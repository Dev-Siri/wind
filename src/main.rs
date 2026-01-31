use serenity::prelude::*;
use shuttle_runtime::SecretStore;

use crate::config::load_json_config;

mod commands;
mod config;
mod event_handler;
mod media;
mod utils;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secrets.get("DISCORD_TOKEN").unwrap();

    let intents = GatewayIntents::all();

    let client = Client::builder(&token, intents)
        .event_handler(event_handler::Handler {
            config: load_json_config().await?,
        })
        .await
        .expect("Error creating serenity client.");

    Ok(client.into())
}
