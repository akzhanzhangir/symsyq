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
                let token = env::var("REGISTER").expect("Expected token for registartion");
                if token == "false" {
                    return Ok(());
                }
                let guild_id = env::var("GUILD_ID").expect("Expected guild ID");
                let mut commands_builder = serenity::builder::CreateApplicationCommands::default();
                let commands = &_framework.options().commands;

                for cmd in commands {
                    commands_builder.add_application_command(cmd.create_as_slash_command().unwrap());
                }
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
