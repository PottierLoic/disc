mod commands;
mod events;
mod reaction_roles;
mod types;

use dotenvy::dotenv;
use std::env;
use types::Data;
use poise::serenity_prelude as serenity;
use serenity::{GatewayIntents, Client};
use poise::Framework;
use events::Handler;

#[tokio::main]
async fn main() {
  dotenv().ok();

  tracing_subscriber::fmt()
    .with_env_filter("info")
    .init();

  let token = env::var("DISCORD_TOKEN").expect("Missing token");
  let guild_id = env::var("GUILD_ID")
    .ok()
    .and_then(|id| id.parse().ok())
    .map(serenity::GuildId::new);

  let framework = Framework::builder()
    .options(poise::FrameworkOptions {
      commands: vec![
        commands::ping(),
        commands::echo(),
        commands::post_roles(),
      ],
      prefix_options: poise::PrefixFrameworkOptions {
        prefix: Some("!".into()),
        ..Default::default()
      },
      ..Default::default()
    })
    .setup(move |ctx, ready, framework| {
      Box::pin(async move {
        println!("Connected as {}", ready.user.name);

        if let Some(guild_id) = guild_id {
          poise::builtins::register_in_guild(ctx, framework.options().commands.as_slice(), guild_id).await?;
          reaction_roles::post_roles_if_missing(ctx, guild_id).await?;
        }

        Ok(Data)
      })
    })
    .build();

  let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

  let mut client = Client::builder(&token, intents)
    .framework(framework)
    .event_handler(Handler)
    .await
    .expect("Client creation failed");

  client.start().await.expect("Bot start failed");
}
