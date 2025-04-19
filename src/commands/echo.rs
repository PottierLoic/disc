use crate::types::Context;

#[poise::command(slash_command, prefix_command)]
pub async fn echo(ctx: Context<'_>, text: String) -> Result<(), anyhow::Error> {
  ctx.say(text).await?;
  Ok(())
}
