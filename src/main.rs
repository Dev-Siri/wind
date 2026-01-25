use anyhow::Result;
use dotenv::dotenv;
use serenity::prelude::*;

mod config;
mod event_handler;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let (_, token) = config::get_discord_creds()?;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(event_handler::Handler)
        .await?;

    client.start().await?;
    Ok(())
}
