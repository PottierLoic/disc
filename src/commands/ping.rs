use crate::types::Context;

#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), anyhow::Error> {
  ctx.say("Pong!").await?;
  Ok(())
}
