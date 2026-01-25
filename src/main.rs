use anyhow::Result;
use dotenv::dotenv;
use serenity::prelude::*;

use crate::config::load_json_config;

mod commands;
mod config;
mod event_handler;
mod media;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let (_, token) = config::get_discord_creds()?;
    env_logger::init();

    let intents = GatewayIntents::all();

    let mut client = Client::builder(&token, intents)
        .event_handler(event_handler::Handler {
            config: load_json_config().await?,
        })
        .await?;

    client.start().await?;
    Ok(())
}
