use serenity::all::Ready;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands;
use crate::config::Config;

#[derive(Debug)]
pub struct Handler {
    pub config: Config,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        log::info!("the wind ({}) is blowing on Discord.", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with("!") {
            if let Err(err) = commands::blame(&ctx, &msg, &self.config).await {
                log::error!("EventHandler.message failed: {:?}", err);
            }
            if let Err(err) = commands::ensure_mute(ctx, msg).await {
                log::error!("EventHandler.message failed: {:?}", err);
            }
            return;
        }

        let msg_content = msg.content.strip_prefix("!").unwrap_or_default();
        let cmd = msg_content
            .split_whitespace()
            .collect::<Vec<&str>>()
            .first()
            .cloned()
            .unwrap_or_default();

        let res = match cmd {
            "ping" => commands::ping(ctx, msg).await,
            "sybau" => commands::sybau(ctx, msg).await,
            "syfm" => commands::syfm(ctx, msg, &self.config).await,
            "rules" => commands::constitution(ctx, msg, &self.config).await,
            "blow" => commands::blow_away(ctx, msg, &self.config).await,
            "src" => commands::source(ctx, msg).await,
            "slime" => commands::slime(ctx, msg, &self.config).await,
            "announce" => commands::announce(ctx, msg, &self.config).await,
            _ => Ok(()),
        };

        if let Err(err) = res {
            log::error!("EventHandler.message failed: {:?}", err);
        }
    }
}
