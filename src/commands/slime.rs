use anyhow::Result;
use serenity::all::{Context, Message};

use crate::{config::Config, utils::is_allowed_operation};

pub async fn slime(ctx: Context, msg: Message, config: &Config) -> Result<()> {
    if !is_allowed_operation(&ctx, &msg, config, false).await? {
        return Ok(());
    }

    for mention in msg.mentions {
        log::info!("Slimed {}", mention.id);
        msg.guild_id.unwrap().ban(&ctx.http, mention.id, 0).await?;
        msg.channel_id
            .say(
                &ctx.http,
                format!("<@!{}> was slimed", mention.id.to_string()),
            )
            .await?;
    }

    Ok(())
}
