use anyhow::Result;
use serenity::all::{Context, GuildId, RoleId, User};

use crate::config::Config;

pub async fn is_allowed_operation(ctx: &Context, user: &User, config: &Config) -> Result<bool> {
    let guild_id = GuildId::new(config.server_id);
    for role in &config.privileged_roles {
        let role_id = RoleId::new(*role);
        if user.has_role(ctx, guild_id, role_id).await? {
            return Ok(true);
        }
    }

    Ok(false)
}
