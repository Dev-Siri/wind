use anyhow::Result;
use serenity::all::{Context, Message};

use crate::{config::Config, utils::is_allowed_operation};

pub async fn blow_away(ctx: Context, msg: Message, config: &Config) -> Result<()> {
    if !is_allowed_operation(&ctx, &msg.author, config).await? {
        return Ok(());
    }

    let ref_msg = match msg.referenced_message {
        Some(m) => m,
        None => return Ok(()),
    };

    ref_msg
        .reply(&ctx, "this message was blown away by the wind.")
        .await?;
    ref_msg.delete(&ctx).await?;
    Ok(())
}
