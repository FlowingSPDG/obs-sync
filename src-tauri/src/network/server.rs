use crate::sync::protocol::SyncMessage;
use anyhow::{Context, Result};
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

type ClientId = String;
type ClientConnection = WebSocketStream<TcpStream>;

pub struct MasterServer {
    clients: Arc<RwLock<HashMap<ClientId, mpsc::UnboundedSender<Message>>>>,
    port: u16,
}

impl MasterServer {
    pub fn new(port: u16) -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            port,
        }
    }

    pub async fn start(&self, mut sync_rx: mpsc::UnboundedReceiver<SyncMessage>) -> Result<()> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr)
            .await
            .context(format!("Failed to bind to {}", addr))?;

        println!("Master server listening on: {}", addr);

        let clients = self.clients.clone();

        // Broadcast sync messages to all connected clients
        tokio::spawn(async move {
            while let Some(message) = sync_rx.recv().await {
                let json = match serde_json::to_string(&message) {
                    Ok(j) => j,
                    Err(e) => {
                        eprintln!("Failed to serialize sync message: {}", e);
                        continue;
                    }
                };

                let clients_lock = clients.read().await;
                for (client_id, tx) in clients_lock.iter() {
                    if let Err(e) = tx.send(Message::Text(json.clone())) {
                        eprintln!("Failed to send message to client {}: {}", client_id, e);
                    }
                }
            }
        });

        // Accept incoming connections
        let clients_for_accept = self.clients.clone();
        tokio::spawn(async move {
            while let Ok((stream, addr)) = listener.accept().await {
                println!("New connection from: {}", addr);
                let clients = clients_for_accept.clone();
                tokio::spawn(handle_connection(stream, addr.to_string(), clients));
            }
        });

        Ok(())
    }

    pub async fn get_connected_clients_count(&self) -> usize {
        self.clients.read().await.len()
    }
}

async fn handle_connection(
    stream: TcpStream,
    client_id: ClientId,
    clients: Arc<RwLock<HashMap<ClientId, mpsc::UnboundedSender<Message>>>>,
) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("WebSocket handshake failed for {}: {}", client_id, e);
            return;
        }
    };

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Add client to the list
    clients.write().await.insert(client_id.clone(), tx);

    // Forward messages from tx to WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if ws_sender.send(message).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages from client (heartbeats, etc.)
    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Close(_)) => break,
            Ok(Message::Ping(data)) => {
                // Send pong
                if let Some(tx) = clients.read().await.get(&client_id) {
                    let _ = tx.send(Message::Pong(data));
                }
            }
            Err(e) => {
                eprintln!("WebSocket error for {}: {}", client_id, e);
                break;
            }
            _ => {}
        }
    }

    // Remove client from the list
    clients.write().await.remove(&client_id);
    send_task.abort();
    println!("Client disconnected: {}", client_id);
}
