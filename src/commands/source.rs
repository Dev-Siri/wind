use anyhow::Result;
use serenity::all::{Context, Message};

use crate::config;

pub async fn source(ctx: Context, msg: Message) -> Result<()> {
    msg.channel_id.say(&ctx, config::PROJECT_SRC).await?;
    Ok(())
}
