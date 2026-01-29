use anyhow::Result;
use serenity::all::{Context, Message, RoleId};

use crate::config::Config;

pub async fn is_allowed_operation(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    allow_controllers: bool,
) -> Result<bool> {
    for role in &config.privileged_roles {
        let role_id = RoleId::new(*role);
        if msg
            .author
            .has_role(ctx, msg.guild_id.unwrap(), role_id)
            .await?
        {
            return Ok(true);
        }
    }

    if allow_controllers {
        for role in &config.controller_roles {
            let role_id = RoleId::new(*role);
            if msg
                .author
                .has_role(ctx, msg.guild_id.unwrap(), role_id)
                .await?
            {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
