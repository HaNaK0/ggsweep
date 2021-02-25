use serde::Deserialize;

/// Config file for the gerator states that generates assets that is generated
#[derive(Deserialize, Debug, Clone)]
pub struct PipelineConfig {
    pub font: String,
    pub target: String,
    pub size: f32,
    pub game_config_path: String,
}
