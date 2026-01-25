use anyhow::Result;
use serenity::all::{Context, Message};

use crate::config::Config;

pub async fn blame(ctx: Context, msg: Message, config: &Config) -> Result<()> {
    let blame = match &config.blame {
        Some(cfg) => cfg,
        None => return Ok(()),
    };

    if blame.phrases.contains(&msg.content.to_lowercase()) {
        log::info!("{} blamed the bot.", msg.author.name);
        msg.reply(ctx, blame.response.clone()).await?;
    }

    Ok(())
}
