use std::fs;

use anyhow::Result;
use serenity::all::{ChannelId, Context, CreateEmbed, CreateMessage, Message};

use crate::{config::Config, utils::is_allowed_operation};

pub async fn constitution(ctx: Context, msg: Message, config: &Config) -> Result<()> {
    if !is_allowed_operation(&ctx, &msg, config, false).await? {
        return Ok(());
    }

    log::info!("Recognized Admin: Sent rule command.");
    let rule_channel_id = ChannelId::new(config.rules_channel_id);

    let rules_file_contents = fs::read_to_string(config.rules.clone())?;
    let rules_txt: Vec<String> = serde_json::from_str(&rules_file_contents)?;
    let mut formatted_rules = String::new();

    for (i, rule) in rules_txt.into_iter().enumerate() {
        formatted_rules += format!("{}. {}\n", i + 1, rule).as_str();
    }

    let embed = CreateEmbed::new()
        .title(config.rules_title.clone())
        .description(formatted_rules);
    let rule_msg = CreateMessage::new().add_embed(embed);

    rule_channel_id.send_message(&ctx, rule_msg).await?;
    Ok(())
}
