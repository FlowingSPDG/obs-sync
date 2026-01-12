use crate::obs::client::{OBSClient, OBSConnectionConfig, OBSConnectionStatus};
use crate::sync::protocol::SyncTargetType;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppMode {
    Master,
    Slave,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
}

pub struct AppState {
    pub obs_client: Arc<OBSClient>,
    pub mode: Arc<RwLock<Option<AppMode>>>,
    pub network_port: Arc<RwLock<u16>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            obs_client: Arc::new(OBSClient::new()),
            mode: Arc::new(RwLock::new(None)),
            network_port: Arc::new(RwLock::new(8080)),
        }
    }
}

#[tauri::command]
pub async fn connect_obs(
    state: State<'_, AppState>,
    config: OBSConnectionConfig,
) -> Result<(), String> {
    state
        .obs_client
        .connect(config)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn disconnect_obs(state: State<'_, AppState>) -> Result<(), String> {
    state
        .obs_client
        .disconnect()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_obs_status(state: State<'_, AppState>) -> Result<OBSConnectionStatus, String> {
    Ok(state.obs_client.get_status().await)
}

#[tauri::command]
pub async fn set_app_mode(state: State<'_, AppState>, mode: AppMode) -> Result<(), String> {
    *state.mode.write().await = Some(mode);
    Ok(())
}

#[tauri::command]
pub async fn get_app_mode(state: State<'_, AppState>) -> Result<Option<AppMode>, String> {
    Ok(state.mode.read().await.clone())
}

#[tauri::command]
pub async fn start_master_server(
    state: State<'_, AppState>,
    port: u16,
) -> Result<(), String> {
    *state.network_port.write().await = port;
    // Master server startup logic would be implemented here
    // This is a placeholder
    Ok(())
}

#[tauri::command]
pub async fn stop_master_server(_state: State<'_, AppState>) -> Result<(), String> {
    // Master server stop logic would be implemented here
    Ok(())
}

#[tauri::command]
pub async fn connect_to_master(
    _state: State<'_, AppState>,
    config: NetworkConfig,
) -> Result<(), String> {
    // Slave client connection logic would be implemented here
    println!("Connecting to master at {}:{}", config.host, config.port);
    Ok(())
}

#[tauri::command]
pub async fn disconnect_from_master(_state: State<'_, AppState>) -> Result<(), String> {
    // Slave client disconnect logic would be implemented here
    Ok(())
}

#[tauri::command]
pub async fn set_sync_targets(
    _state: State<'_, AppState>,
    targets: Vec<SyncTargetType>,
) -> Result<(), String> {
    println!("Setting sync targets: {:?}", targets);
    // Sync targets configuration would be implemented here
    Ok(())
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
