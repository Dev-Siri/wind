use anyhow::Result;
use serenity::all::{Context, CreateAttachment, CreateMessage, Message};

use crate::media;

// This is some next-gen commands you won't get in your regular discord bot.
pub async fn sybau(ctx: Context, msg: Message) -> Result<()> {
    for mention in msg.mentions {
        let sybau_gif = CreateAttachment::url(&ctx.http, media::SYBAU_GIF_URL).await?;
        let reply = CreateMessage::new()
            .add_file(sybau_gif)
            .content(format!("Sybau <@!{}>", mention.id.to_string()));
        msg.channel_id.send_message(&ctx.http, reply).await?;
    }

    Ok(())
}
