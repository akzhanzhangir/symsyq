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
        ..Default::default()
    };
    poise::Framework::build()
        .token(token)
        .options(options)
        .command(play(), |f| f)
        .command(pause(), |f| f)
        .command(resume(), |f| f)
        .command(skip(), |f| f)
        .command(clear(), |f| f)
        .command(disconnect(), |f| f)
        .command(song(), |f| f)
        .command(songqueue(), |f| f)
        .command(repeat(), |f| f)
        .command(switch(), |f| f)
        .command(remove(), |f| f)
        .command(seek(), |f| f)
        .client_settings(|client| client.register_songbird())
        .user_data_setup(move |_ctx, _ready, _framework| {
            // This part of for registering the commands on the start of the bot
            Box::pin(async move {
                let token = env::var("REGISTER").expect("Expected token for registartion");
                if token == "false" {
                    return Ok(());
                }
                let guild_id = env::var("GUILD_ID").expect("Expected guild ID");
                let mut commands_builder =
                    poise::serenity_prelude::CreateApplicationCommands::default();
                let commands = &_framework.options().application_options.commands;

                for cmd in commands {
                    let desc;
                    match cmd {
                        poise::ApplicationCommandTree::Slash(slash_command) => {
                            desc = slash_command.description();
                            println!("{}", desc);
                            println!("{}", slash_command.name());
                        }
                        poise::ApplicationCommandTree::ContextMenu(_) => todo!(),
                    }
                    commands_builder
                        .create_application_command(|f| cmd.create(f).description(desc));
                }
                let commands_builder = serenity::json::Value::Array(commands_builder.0);
                _ctx.http
                    .create_guild_application_commands(
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
