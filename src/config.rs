use std::env;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server_id: u64,
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
const CLIENT_ID_KEY: &str = "DISCORD_CLIENT_ID";
const TOKEN_KEY: &str = "DISCORD_TOKEN";
const CONFIG_FILE_NAME: &str = "wind.json";

pub fn get_discord_creds() -> Result<(String, String)> {
    let client_id = env::var(CLIENT_ID_KEY)
        .map_err(|e| anyhow!("Unable to retrieve DISCORD_CLIENT_ID: {:?}", e))?;
    let token = env::var(TOKEN_KEY)
        .map_err(|e| anyhow!("Unable to retrieve DISCORD_CLIENT_ID: {:?}", e))?;

    Ok((client_id, token))
}

pub async fn load_json_config() -> Result<Config> {
    let config_file_contents = fs::read_to_string(CONFIG_FILE_NAME).await?;
    let config: Config = serde_json::from_str(&config_file_contents)?;

    Ok(config)
}
