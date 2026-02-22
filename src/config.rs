use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server_id: u64,
    pub controller_roles: Vec<u64>,
    pub privileged_roles: Vec<u64>,
    pub blame: Option<BlameConfig>,
    pub rules_channel_id: u64,
    pub announce_channel_id: u64,
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
    let config_file_contents = fs::read(CONFIG_FILE_NAME)
        .await
        .map_err(|e| anyhow!("Failed to load config file: {}", e))?;
    let config: Config = serde_json::from_slice(&config_file_contents)?;

    Ok(config)
}
