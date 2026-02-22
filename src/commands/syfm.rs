use anyhow::Result;
use serenity::all::{Context, CreateMessage, Mentionable, Message};
use tokio::fs;

use crate::{config::Config, utils::is_allowed_operation};

const MUTE_FILE_PATH: &str = "mute.json";

async fn get_muted() -> Result<Vec<u64>> {
    let mute_file = fs::read(MUTE_FILE_PATH).await?;
    let parsed_mute = serde_json::from_slice(&mute_file)?;

    Ok(parsed_mute)
}

async fn add_to_muted(muted_uid: u64) -> Result<()> {
    let mute_file = fs::read(MUTE_FILE_PATH).await?;
    let mut parsed_mute: Vec<u64> = serde_json::from_slice(&mute_file)?;
    parsed_mute.push(muted_uid);

    let updated_mute_list = serde_json::to_vec(&parsed_mute)?;
    fs::write(MUTE_FILE_PATH, updated_mute_list).await?;

    Ok(())
}

async fn remove_muted(muted_uid: u64) -> Result<()> {
    let mute_file = fs::read(MUTE_FILE_PATH).await?;
    let parsed_mute: Vec<u64> = serde_json::from_slice(&mute_file)?;
    let filtered_mute: Vec<&u64> = parsed_mute.iter().filter(|m| **m != muted_uid).collect();

    let updated_mute_list = serde_json::to_vec(&filtered_mute)?;
    fs::write(MUTE_FILE_PATH, updated_mute_list).await?;

    Ok(())
}

pub async fn ensure_mute(ctx: Context, msg: Message) -> Result<()> {
    let muted_list = get_muted().await?;

    if muted_list.contains(&msg.author.id.get()) {
        log::info!("Syfm-ed user's message revoked.");
        msg.delete(&ctx.http).await?;
    }

    Ok(())
}

pub async fn syfm(ctx: Context, msg: Message, config: &Config) -> Result<()> {
    if !is_allowed_operation(&ctx, &msg, config, false).await? {
        return Ok(());
    }

    let Some(syfmed_target) = msg.mentions.first() else {
        return Ok(());
    };

    let muted_uid = syfmed_target.id.get();
    let muted = get_muted().await?;
    if muted.contains(&muted_uid) {
        remove_muted(muted_uid).await?;
    } else {
        add_to_muted(muted_uid).await?;
    }

    let reply_msg = CreateMessage::new().content(format!("syfm {}", syfmed_target.mention()));

    msg.channel_id.send_message(&ctx.http, reply_msg).await?;

    Ok(())
}
