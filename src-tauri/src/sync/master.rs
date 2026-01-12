use super::protocol::{SyncMessage, SyncMessageType, SyncTargetType, StateSyncPayload, SceneInfo};
use crate::obs::{events::OBSEvent, commands::OBSCommands, OBSClient};
use anyhow::Result;
use std::sync::Arc;
use std::path::Path;
use tokio::sync::{mpsc, RwLock};
use tokio::fs;

pub struct MasterSync {
    obs_client: Arc<OBSClient>,
    message_tx: mpsc::UnboundedSender<SyncMessage>,
    active_targets: Arc<RwLock<Vec<SyncTargetType>>>,
}

impl MasterSync {
    pub fn new(obs_client: Arc<OBSClient>) -> (Self, mpsc::UnboundedReceiver<SyncMessage>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (
            Self {
                obs_client,
                message_tx: tx,
                active_targets: Arc::new(RwLock::new(vec![
                    SyncTargetType::Program,
                    SyncTargetType::Source,
                ])),
            },
            rx,
        )
    }

    pub async fn set_active_targets(&self, targets: Vec<SyncTargetType>) {
        *self.active_targets.write().await = targets;
    }

    pub async fn start_monitoring(&self, mut obs_event_rx: mpsc::UnboundedReceiver<OBSEvent>) {
        let message_tx = self.message_tx.clone();
        let active_targets = self.active_targets.clone();

        tokio::spawn(async move {
            while let Some(event) = obs_event_rx.recv().await {
                let targets = active_targets.read().await.clone();

                match event {
                    OBSEvent::SceneChanged { scene_name } => {
                        if targets.contains(&SyncTargetType::Program) {
                            let payload = serde_json::json!({
                                "scene_name": scene_name
                            });
                            let msg = SyncMessage::new(
                                SyncMessageType::SceneChange,
                                SyncTargetType::Program,
                                payload,
                            );
                            let _ = message_tx.send(msg);
                        }
                    }
                    OBSEvent::CurrentPreviewSceneChanged { scene_name } => {
                        if targets.contains(&SyncTargetType::Preview) {
                            let payload = serde_json::json!({
                                "scene_name": scene_name
                            });
                            let msg = SyncMessage::new(
                                SyncMessageType::SceneChange,
                                SyncTargetType::Preview,
                                payload,
                            );
                            let _ = message_tx.send(msg);
                        }
                    }
                    OBSEvent::SceneItemTransformChanged {
                        scene_name,
                        scene_item_id,
                    } => {
                        if targets.contains(&SyncTargetType::Source) {
                            let payload = serde_json::json!({
                                "scene_name": scene_name,
                                "scene_item_id": scene_item_id
                            });
                            let msg = SyncMessage::new(
                                SyncMessageType::TransformUpdate,
                                SyncTargetType::Source,
                                payload,
                            );
                            let _ = message_tx.send(msg);
                        }
                    }
                    OBSEvent::InputSettingsChanged { input_name } => {
                        if targets.contains(&SyncTargetType::Source) {
                            let payload = serde_json::json!({
                                "input_name": input_name
                            });
                            let msg = SyncMessage::new(
                                SyncMessageType::ImageUpdate,
                                SyncTargetType::Source,
                                payload,
                            );
                            let _ = message_tx.send(msg);
                        }
                    }
                    _ => {}
                }
            }
        });
    }

    pub fn send_heartbeat(&self) -> Result<()> {
        self.message_tx.send(SyncMessage::heartbeat())?;
        Ok(())
    }
    
    /// Slave接続時に現在の状態を送信
    pub async fn send_initial_state(&self) -> Result<()> {
        let client_arc = self.obs_client.get_client_arc();
        let client_lock = client_arc.read().await;
        let client = client_lock.as_ref().ok_or_else(|| anyhow::anyhow!("OBS not connected"))?;
        
        // 現在のプログラムシーンを取得
        let current_program_scene = OBSCommands::get_current_program_scene(client).await?;
        
        // 現在のプレビューシーンを取得
        let current_preview_scene = OBSCommands::get_current_preview_scene(client).await?;
        
        // すべてのシーンとそのアイテムを取得
        let scenes = self.get_all_scenes_info(client).await?;
        
        let payload = StateSyncPayload {
            current_program_scene,
            current_preview_scene,
            scenes,
        };
        
        let message = SyncMessage::new(
            SyncMessageType::StateSync,
            SyncTargetType::Program,
            serde_json::to_value(payload)?,
        );
        
        self.message_tx.send(message)?;
        Ok(())
    }
    
    /// すべてのシーン情報を取得
    async fn get_all_scenes_info(&self, _client: &obws::Client) -> Result<Vec<SceneInfo>> {
        // Note: obwsライブラリの実際のAPIに合わせて実装が必要
        // 現状はプレースホルダー
        let scenes = vec![];
        
        // 実際の実装では：
        // 1. client.scenes().list()でシーン一覧取得
        // 2. 各シーンのアイテムを取得
        // 3. 画像ソースの場合、ファイルパスを取得してバイナリデータを読み込む
        
        Ok(scenes)
    }
    
    /// 画像ファイルを読み込んでバイナリデータを取得
    pub async fn read_image_file(file_path: &str) -> Result<Vec<u8>> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(anyhow::anyhow!("Image file not found: {}", file_path));
        }
        
        let data = fs::read(path).await?;
        Ok(data)
    }
}
