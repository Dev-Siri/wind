use anyhow::Result;
use serenity::all::{ChannelId, Context, CreateEmbed, CreateMessage, Message};

use crate::{config::Config, utils::is_allowed_operation};

pub async fn announce(ctx: Context, msg: Message, config: &Config) -> Result<()> {
    if !is_allowed_operation(&ctx, &msg, config, true).await? {
        return Ok(());
    }

    if !msg.content.contains(" ") {
        log::info!("Message is not in valid format. Skipping announcement.");
        return Ok(());
    }

    let msg_parts: Vec<&str> = msg
        .content
        .strip_prefix("!announce")
        .unwrap_or_default()
        .trim()
        .split(":")
        .collect();

    let (Some(title), Some(content)) = (msg_parts.get(0), msg_parts.get(1)) else {
        log::info!("Not enough arguments for announcement.");
        return Ok(());
    };

    let announce_channel = ChannelId::new(config.announce_channel_id);

    let embed = CreateEmbed::new()
        .title(format!("📢 {}", title))
        .description(content.to_owned());
    let announcement = CreateMessage::new().add_embed(embed);

    announce_channel
        .send_message(&ctx.http, announcement)
        .await?;

    Ok(())
}
