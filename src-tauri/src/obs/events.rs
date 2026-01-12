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
    _event_tx: mpsc::UnboundedSender<OBSEvent>,
}

impl OBSEventHandler {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<OBSEvent>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { _event_tx: tx }, rx)
    }

    #[allow(dead_code)]
    pub async fn start_listening(&self, _client: &Client) -> anyhow::Result<()> {
        // Event listening would be implemented based on the actual obws API
        // This is a placeholder for now
        Ok(())
    }
}

impl Default for OBSEventHandler {
    fn default() -> Self {
        Self::new().0
    }
}
