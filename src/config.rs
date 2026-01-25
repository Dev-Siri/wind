use std::env;

use anyhow::{Result, anyhow};

const CLIENT_ID_KEY: &str = "DISCORD_CLIENT_ID";
const TOKEN_KEY: &str = "DISCORD_TOKEN";

pub fn get_discord_creds() -> Result<(String, String)> {
    let client_id = env::var(CLIENT_ID_KEY)
        .map_err(|e| anyhow!("Unable to retrieve DISCORD_CLIENT_ID: {:?}", e))?;
    let token = env::var(TOKEN_KEY)
        .map_err(|e| anyhow!("Unable to retrieve DISCORD_CLIENT_ID: {:?}", e))?;

    Ok((client_id, token))
}
