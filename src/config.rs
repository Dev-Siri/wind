use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server_id: u64,
    pub controller_roles: Vec<u64>,
    pub privileged_roles: Vec<u64>,
    pub blame: Option<BlameConfig>,
    pub rules_channel_id: u64,
    pub rules_title: String,
    pub rules: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlameConfig {
    pub response: String,
    pub phrases: Vec<String>,
}

pub const PROJECT_SRC: &str = "https://github.com/Dev-Siri/wind";
const CONFIG_FILE_NAME: &str = "wind.json";

pub async fn load_json_config() -> Result<Config> {
    let config_file_contents = fs::read_to_string(CONFIG_FILE_NAME).await?;
    let config: Config = serde_json::from_str(&config_file_contents)?;

    Ok(config)
}
