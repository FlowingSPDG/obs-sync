use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SyncMessageType {
    SourceUpdate,
    TransformUpdate,
    SceneChange,
    ImageUpdate,
    Heartbeat,
    StateSync,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SyncTargetType {
    Source,
    Preview,
    Program,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMessage {
    #[serde(rename = "type")]
    pub message_type: SyncMessageType,
    pub timestamp: i64,
    pub target_type: SyncTargetType,
    pub payload: Value,
}

impl SyncMessage {
    pub fn new(message_type: SyncMessageType, target_type: SyncTargetType, payload: Value) -> Self {
        Self {
            message_type,
            timestamp: chrono::Utc::now().timestamp_millis(),
            target_type,
            payload,
        }
    }

    pub fn heartbeat() -> Self {
        Self::new(
            SyncMessageType::Heartbeat,
            SyncTargetType::Program,
            Value::Object(serde_json::Map::new()),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformUpdatePayload {
    pub scene_name: String,
    pub scene_item_id: i64,
    pub transform: TransformData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformData {
    pub position_x: f64,
    pub position_y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneChangePayload {
    pub scene_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUpdatePayload {
    pub scene_name: String,
    pub source_name: String,
    pub file: String,
    pub width: Option<f64>,
    pub height: Option<f64>,
}
