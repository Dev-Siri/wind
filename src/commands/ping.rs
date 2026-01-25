use anyhow::Result;
use serenity::all::{Context, Message};

pub async fn ping(ctx: Context, msg: Message) -> Result<()> {
    msg.channel_id
        .say(&ctx.http, "the wind blows a pong")
        .await?;
    Ok(())
}
