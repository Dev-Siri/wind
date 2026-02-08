use anyhow::{Result, anyhow};
use dotenv::dotenv;
use serenity::prelude::*;
use tokio::fs;

use crate::config::load_json_config;

mod commands;
mod config;
mod event_handler;
mod media;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    if fs::try_exists(".env").await? {
        dotenv().ok();
    }

    let token = std::env::var("DISCORD_TOKEN").map_err(|_| anyhow!("Missing DISCORD_TOKEN."))?;

    let intents = GatewayIntents::all();

    let mut client = Client::builder(&token, intents)
        .event_handler(event_handler::Handler {
            config: load_json_config().await?,
        })
        .await?;

    client.start().await?;
    Ok(())
}
