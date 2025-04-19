use serenity::{
  async_trait,
  model::prelude::*,
  prelude::*,
};
use crate::reaction_roles::load_role_mappings;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn reaction_add(&self, ctx: Context, add: Reaction) {
    super::handle_reaction(ctx, add, true).await;
  }

  async fn reaction_remove(&self, ctx: Context, remove: Reaction) {
    super::handle_reaction(ctx, remove, false).await;
  }
}

pub async fn handle_reaction(ctx: Context, reaction: Reaction, add: bool) {
  if reaction.user_id.is_none() || reaction.guild_id.is_none() {
    return;
  }

  let emoji = match &reaction.emoji {
    ReactionType::Unicode(e) => e.clone(),
    _ => return,
  };

  let mappings = match load_role_mappings() {
    Ok(m) => m,
    Err(_) => return,
  };
  
  let role_id = match mappings.iter().find(|m| m.emoji == emoji) {
    Some(m) => RoleId::new(m.role_id),
    None => return,
  };

  let guild_id = reaction.guild_id.unwrap();
  let user_id = reaction.user_id.unwrap();
  let bot_id = ctx.cache.current_user().id;

  if user_id == bot_id {
    return;
  }

  if let Ok(member) = guild_id.member(&ctx.http, user_id).await {
    let _ = if add {
      member.add_role(&ctx.http, role_id).await
    } else {
      member.remove_role(&ctx.http, role_id).await
    };
  }
}
