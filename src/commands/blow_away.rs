use anyhow::Result;
use chrono::{Duration, Utc};
use serenity::all::{Context, Message};

use crate::{config::Config, utils::is_allowed_operation};

pub async fn blow_away(ctx: Context, msg: Message, config: &Config) -> Result<()> {
    if !is_allowed_operation(&ctx, &msg, config, true).await? {
        return Ok(());
    }

    let ref_msg = match msg.referenced_message {
        Some(m) => m,
        None => return Ok(()),
    };

    let sent_at = ref_msg.timestamp.to_utc();
    let now = Utc::now();

    // Only allow for blowing away messages posted within 20 minutes.
    let is_within_20_minutes = now.signed_duration_since(sent_at) <= Duration::minutes(20);
    if is_within_20_minutes {
        log::info!("Blowing away message {}.", ref_msg.id);
        ref_msg
            .reply(&ctx, "this message was blown away by the wind.")
            .await?;
        ref_msg.delete(&ctx).await?;
    }

    Ok(())
}
