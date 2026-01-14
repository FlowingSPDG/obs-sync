use obws::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum OBSEvent {
    SceneChanged { scene_name: String },
    SceneItemTransformChanged { scene_name: String, scene_item_id: i64 },
    SourceCreated { source_name: String },
    SourceDestroyed { source_name: String },
    InputSettingsChanged { input_name: String },
    CurrentPreviewSceneChanged { scene_name: String },
}

pub struct OBSEventHandler {
    event_tx: mpsc::UnboundedSender<OBSEvent>,
}

impl OBSEventHandler {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<OBSEvent>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { event_tx: tx }, rx)
    }

    pub async fn start_listening(&self, client: &Client) -> anyhow::Result<()> {
        let tx = self.event_tx.clone();

        // Get event stream from obws client
        let events = client.events();

        tokio::spawn(async move {
            println!("✓ Started OBS event listening");

            loop {
                match events.recv().await {
                    Ok(event) => {
                        // Map obws events to our internal event type
                        let obs_event = match event {
                            obws::events::Event::CurrentProgramSceneChanged { scene_name } => {
                                println!("Event: Scene changed to {}", scene_name);
                                Some(OBSEvent::SceneChanged { scene_name })
                            }
                            obws::events::Event::SceneItemTransformChanged { scene_name, scene_item_id } => {
                                println!("Event: Transform changed for item {} in scene {}", scene_item_id, scene_name);
                                Some(OBSEvent::SceneItemTransformChanged {
                                    scene_name,
                                    scene_item_id
                                })
                            }
                            obws::events::Event::InputCreated { input_name, .. } => {
                                println!("Event: Source created: {}", input_name);
                                Some(OBSEvent::SourceCreated { source_name: input_name })
                            }
                            obws::events::Event::InputRemoved { input_name } => {
                                println!("Event: Source destroyed: {}", input_name);
                                Some(OBSEvent::SourceDestroyed { source_name: input_name })
                            }
                            obws::events::Event::InputSettingsChanged { input_name, .. } => {
                                println!("Event: Input settings changed: {}", input_name);
                                Some(OBSEvent::InputSettingsChanged { input_name })
                            }
                            obws::events::Event::CurrentPreviewSceneChanged { scene_name } => {
                                println!("Event: Preview scene changed to {}", scene_name);
                                Some(OBSEvent::CurrentPreviewSceneChanged { scene_name })
                            }
                            _ => {
                                // Ignore other event types
                                None
                            }
                        };

                        // Send mapped event to the channel
                        if let Some(evt) = obs_event {
                            if let Err(e) = tx.send(evt) {
                                eprintln!("Failed to send OBS event: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving OBS event: {}", e);
                        break;
                    }
                }
            }

            println!("OBS event listener stopped");
        });

        Ok(())
    }
}

impl Default for OBSEventHandler {
    fn default() -> Self {
        Self::new().0
    }
}
