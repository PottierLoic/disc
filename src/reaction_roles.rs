use serenity::{
  model::prelude::*,
  prelude::*,
  builder::GetMessages,
};
use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct RoleMapping {
  pub emoji: String,
  pub role_id: u64,
  pub label: String,
}

pub fn load_role_mappings() -> anyhow::Result<Vec<RoleMapping>> {
  let file = fs::read_to_string("roles.json")
    .map_err(|e| anyhow::anyhow!("Failed to read roles.json: {}", e))?;
  let data: Vec<RoleMapping> = serde_json::from_str(&file)?;
  Ok(data)
}

pub async fn post_roles_if_missing(ctx: &Context, guild_id: GuildId) -> Result<()> {
  let mappings = load_role_mappings()?;
  let channels = guild_id.channels(&ctx.http).await?;
  let channel = channels
      .values()
      .find(|c| c.name == "roles")
      .ok_or_else(|| anyhow::anyhow!("No #roles channel found"))?;

  let messages = channel
      .messages(&ctx.http, GetMessages::default().limit(10))
      .await?;
  let bot_id = ctx.cache.current_user().id;
  let already_posted = messages.iter().any(|msg| msg.author.id == bot_id);

  if already_posted {
      return Ok(());
  }

  let mut content = String::from("React to get a role:\n\n");
  for mapping in &mappings {
      let role_mention = RoleId::new(mapping.role_id).mention().to_string();
      content.push_str(&format!("{} = {} ({})\n", mapping.emoji, role_mention, mapping.label));
    }

  let message = channel.say(&ctx.http, content).await?;

  for mapping in &mappings {
    message
      .react(&ctx.http, ReactionType::Unicode(mapping.emoji.clone()))
      .await?;
  }

  Ok(())
}
