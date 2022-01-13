use std::env;
mod commands;
use commands::*;
use songbird::SerenityInit;
mod utils;
use utils::*;

type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let options = poise::FrameworkOptions {
        commands: vec![
            play(),
            pause(),
            resume(),
            skip(),
            clear(),
            disconnect(),
            song(),
            songqueue(),
            repeat(),
            switch(),
            remove(),
            seek(),
        ],
        ..Default::default()
    };
    poise::Framework::build()
        .token(token)
        .options(options)
        .client_settings(|client| client.register_songbird())
        .user_data_setup(move |_ctx, _ready, _framework| {
            // This part of for registering the commands on the start of the bot
            Box::pin(async move {
                let guild_id = env::var("GUILD_ID").expect("Expected guild ID");
                let mut commands_builder = serenity::builder::CreateApplicationCommands::default();
                let command_to_reg = register().create_as_slash_command().unwrap();
                commands_builder.add_application_command(command_to_reg);
                let commands_builder = serenity::json::Value::Array(commands_builder.0);
                _ctx.http
                    .create_guild_application_command(
                        guild_id.parse::<u64>().unwrap(),
                        &commands_builder,
                    )
                    .await?;
                Ok(())
            })
        })
        .run()
        .await
        .unwrap();
}

/// Register commands in guild
#[poise::command(slash_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx,false).await?;

    Ok(())
}
