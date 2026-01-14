use anyhow::{Context, Result};
use obws::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneItemTransform {
    pub position_x: f64,
    pub position_y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub width: f64,
    pub height: f64,
}

pub struct OBSCommands;

impl OBSCommands {
    pub async fn get_current_program_scene(client: &Client) -> Result<String> {
        let scene = client
            .scenes()
            .current_program_scene()
            .await
            .context("Failed to get current program scene")?;
        Ok(scene)
    }

    pub async fn get_current_preview_scene(client: &Client) -> Result<Option<String>> {
        match client.scenes().current_preview_scene().await {
            Ok(scene) => Ok(Some(scene)),
            Err(_) => Ok(None),
        }
    }

    pub async fn set_current_program_scene(client: &Client, scene_name: &str) -> Result<()> {
        client
            .scenes()
            .set_current_program_scene(scene_name)
            .await
            .context("Failed to set current program scene")?;
        Ok(())
    }

    pub async fn set_scene_item_transform(
        client: &Client,
        scene_name: &str,
        scene_item_id: i64,
        transform: SceneItemTransform,
    ) -> Result<()> {
        client
            .scene_items()
            .set_transform(obws::requests::scene_items::SetTransform {
                scene: scene_name,
                item_id: scene_item_id,
                position_x: Some(transform.position_x),
                position_y: Some(transform.position_y),
                rotation: Some(transform.rotation),
                scale_x: Some(transform.scale_x),
                scale_y: Some(transform.scale_y),
                ..Default::default()
            })
            .await
            .context("Failed to set scene item transform")?;
        Ok(())
    }

    pub async fn get_scene_item_list(client: &Client, scene_name: &str) -> Result<Vec<Value>> {
        let items = client
            .scene_items()
            .list(scene_name)
            .await
            .context("Failed to get scene item list")?;

        let result = items
            .into_iter()
            .map(|item| {
                serde_json::json!({
                    "scene_item_id": item.id,
                    "source_name": item.source_name,
                    "source_type": item.source_type,
                    "enabled": item.enabled,
                })
            })
            .collect();

        Ok(result)
    }

    pub async fn set_input_settings(
        client: &Client,
        input_name: &str,
        settings: &Value,
    ) -> Result<()> {
        client
            .inputs()
            .set_settings(obws::requests::inputs::SetSettings {
                input: input_name,
                settings,
                overlay: Some(true),
            })
            .await
            .context("Failed to set input settings")?;
        Ok(())
    }
}
