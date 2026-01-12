use super::protocol::{SyncMessage, SyncMessageType, StateSyncPayload, ImageUpdatePayload};
use crate::obs::{commands::OBSCommands, OBSClient};
use anyhow::{Context, Result};
use std::sync::Arc;
use std::path::Path;
use tokio::sync::mpsc;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct DesyncAlert {
    pub id: String,
    pub timestamp: i64,
    pub scene_name: String,
    pub source_name: String,
    pub message: String,
    pub severity: AlertSeverity,
}

#[derive(Debug, Clone)]
pub enum AlertSeverity {
    Warning,
    Error,
}

pub struct SlaveSync {
    obs_client: Arc<OBSClient>,
    alert_tx: mpsc::UnboundedSender<DesyncAlert>,
}

impl SlaveSync {
    pub fn new(obs_client: Arc<OBSClient>) -> (Self, mpsc::UnboundedReceiver<DesyncAlert>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (
            Self {
                obs_client,
                alert_tx: tx,
            },
            rx,
        )
    }

    pub async fn apply_sync_message(&self, message: SyncMessage) -> Result<()> {
        let client_arc = self.obs_client.get_client_arc();
        let client_lock = client_arc.read().await;
        let client = client_lock.as_ref().context("OBS client not connected")?;

        match message.message_type {
            SyncMessageType::SceneChange => {
                let scene_name = message.payload["scene_name"]
                    .as_str()
                    .context("Invalid scene_name in payload")?;

                if let Err(e) = OBSCommands::set_current_program_scene(&client, scene_name).await {
                    self.send_alert(
                        scene_name.to_string(),
                        String::new(),
                        format!("Failed to change scene: {}", e),
                        AlertSeverity::Error,
                    )?;
                }
            }
            SyncMessageType::TransformUpdate => {
                let scene_name = message.payload["scene_name"]
                    .as_str()
                    .context("Invalid scene_name")?;
                let scene_item_id = message.payload["scene_item_id"]
                    .as_i64()
                    .context("Invalid scene_item_id")?;

                // Note: Transform data would need to be included in the payload
                // This is a simplified version
                if let Err(e) = self.handle_transform_update(&client, scene_name, scene_item_id).await {
                    self.send_alert(
                        scene_name.to_string(),
                        String::new(),
                        format!("Failed to update transform: {}", e),
                        AlertSeverity::Warning,
                    )?;
                }
            }
            SyncMessageType::ImageUpdate => {
                let payload: ImageUpdatePayload = serde_json::from_value(message.payload)
                    .context("Invalid ImageUpdatePayload")?;

                let scene_name = payload.scene_name.clone();
                let source_name = payload.source_name.clone();

                // Handle image update with binary data
                if let Err(e) = self.handle_image_update_with_data(&client, payload).await {
                    self.send_alert(
                        scene_name,
                        source_name,
                        format!("Failed to update image: {}", e),
                        AlertSeverity::Warning,
                    )?;
                }
            }
            SyncMessageType::StateSync => {
                let payload: StateSyncPayload = serde_json::from_value(message.payload)
                    .context("Invalid StateSyncPayload")?;

                // Apply initial state
                if let Err(e) = self.apply_initial_state(&client, payload).await {
                    self.send_alert(
                        String::new(),
                        String::new(),
                        format!("Failed to apply initial state: {}", e),
                        AlertSeverity::Error,
                    )?;
                }
            }
            SyncMessageType::Heartbeat => {
                // Just acknowledge heartbeat
            }
            _ => {}
        }

        Ok(())
    }

    async fn handle_transform_update(
        &self,
        _client: &obws::Client,
        _scene_name: &str,
        _scene_item_id: i64,
    ) -> Result<()> {
        // Transform update logic would go here
        Ok(())
    }

    async fn handle_image_update(&self, _client: &obws::Client, _input_name: &str) -> Result<()> {
        // Image update logic would go here
        Ok(())
    }
    
    /// 画像更新（バイナリデータ付き）
    async fn handle_image_update_with_data(
        &self,
        client: &obws::Client,
        payload: ImageUpdatePayload,
    ) -> Result<()> {
        // 1. 一時ディレクトリに画像を保存
        let temp_dir = std::env::temp_dir().join("obs-sync");
        fs::create_dir_all(&temp_dir).await?;
        
        // 元のファイル名を取得
        let file_name = Path::new(&payload.file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("synced_image.png");
        
        let temp_file = temp_dir.join(file_name);
        
        // 2. バイナリデータを書き込み
        fs::write(&temp_file, &payload.image_data).await?;
        
        // 3. OBSの画像ソースを更新
        let settings = serde_json::json!({
            "file": temp_file.to_string_lossy().to_string(),
        });
        
        OBSCommands::set_input_settings(client, &payload.source_name, &settings).await?;
        
        println!(
            "Image synced: {} -> {}",
            payload.source_name,
            temp_file.display()
        );
        
        Ok(())
    }
    
    /// 初期状態を適用（再接続時の同期ズレ回避）
    async fn apply_initial_state(
        &self,
        client: &obws::Client,
        payload: StateSyncPayload,
    ) -> Result<()> {
        println!("Applying initial state from master...");
        
        // 1. プログラムシーンを設定
        OBSCommands::set_current_program_scene(client, &payload.current_program_scene).await?;
        
        // 2. プレビューシーンを設定（存在する場合）
        if let Some(preview_scene) = payload.current_preview_scene {
            // Note: obwsライブラリでプレビューシーン設定が可能な場合
            println!("Preview scene would be set to: {}", preview_scene);
        }
        
        // 3. 各シーンのアイテムを同期
        for scene in payload.scenes {
            for item in scene.items {
                // 画像ソースの場合、画像データを保存して設定
                if let Some(image_data) = item.image_data {
                    if let Some(image_path) = item.image_path {
                        let temp_dir = std::env::temp_dir().join("obs-sync");
                        fs::create_dir_all(&temp_dir).await?;
                        
                        let file_name = Path::new(&image_path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("synced_image.png");
                        
                        let temp_file = temp_dir.join(file_name);
                        fs::write(&temp_file, &image_data).await?;
                        
                        let settings = serde_json::json!({
                            "file": temp_file.to_string_lossy().to_string(),
                        });
                        
                        OBSCommands::set_input_settings(client, &item.source_name, &settings).await?;
                    }
                }
                
                // トランスフォームを設定
                // Note: 実際のAPIに合わせて実装が必要
            }
        }
        
        println!("Initial state applied successfully");
        Ok(())
    }

    fn send_alert(
        &self,
        scene_name: String,
        source_name: String,
        message: String,
        severity: AlertSeverity,
    ) -> Result<()> {
        let alert = DesyncAlert {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            scene_name,
            source_name,
            message,
            severity,
        };
        self.alert_tx.send(alert)?;
        Ok(())
    }
}
