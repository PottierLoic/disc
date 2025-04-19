use crate::{types::Context, reaction_roles::load_role_mappings};
use serenity::model::prelude::ReactionType;

#[poise::command(slash_command)]
pub async fn post_roles(ctx: Context<'_>) -> Result<(), anyhow::Error> {
  let mappings = load_role_mappings()?;

  let msg = ctx
    .channel_id()
    .say(&ctx.http(), "React to get your roles :\n\n🎮 = Gamer\n💻 = Dev\n🎨 = Artist")
    .await?;

  for mapping in &mappings {
    let _ = msg.react(&ctx.http(), ReactionType::Unicode(mapping.emoji.clone())).await;
  }

  Ok(())
}
