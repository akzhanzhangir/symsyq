use crate::{send_message, Context, Error};
use songbird::{error::JoinError, Event, EventContext, EventHandler as VoiceEventHandler};
use std::{sync::Arc, time::Duration};

use serenity::{
    async_trait,
    http::Http,
    model::{prelude::ChannelId, prelude::GuildId},
    prelude::Context as SerenityContext,
};

use poise::ApplicationCommandOrAutocompleteInteraction::*;

/// Command to play a song from link or query
#[poise::command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "A song URL or YouTube search query"] query: String,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            send_message(ctx, "You are not in voice").await;
            return Ok(());
        }
    };

    send_message(ctx, "Loading...").await;

    let manager = songbird::get(ctx.discord())
        .await
        .expect("Songbird Voice client placed in at initialisation")
        .clone();

    let handler_lock;
    let success: Result<(), JoinError>;
    if let Some(call) = manager.get(guild_id) {
        if call.lock().await.current_channel().unwrap().to_string() == connect_to.to_string() {
            handler_lock = call;
            success = Ok(());
        } else {
            return Ok(());
        }
    } else {
        let (h, s) = manager.join(guild_id, connect_to).await;
        handler_lock = h;
        success = s;
    }

    if let Ok(_channel) = success {
        let mut handler = handler_lock.lock().await;
        handler.deafen(true).await?;

        if query.clone().starts_with("http") {
            let source = match songbird::input::restartable::Restartable::ytdl(query, true).await {
                Ok(source) => source,
                Err(why) => {
                    println!("Err starting source: {:?}", why);
                    return Ok(());
                }
            };
            handler.enqueue_source(source.into());
        } else {
            let source =
                match songbird::input::restartable::Restartable::ytdl_search(query, true).await {
                    Ok(source) => source,
                    Err(why) => {
                        println!("Err starting source: {:?}", why);
                        return Ok(());
                    }
                };
            handler.enqueue_source(source.into());
        }
        let cont = ctx.discord().clone();
        handler.add_global_event(
            Event::Periodic(Duration::from_secs(300), None),
            ChannelDurationNotifier {
                chan_id: connect_to,
                guild_id,
                http: cont.http.clone(),
                ctx: cont,
            },
        );

        let queue = handler.queue().current_queue();
        drop(handler);
        let mut _current;
        if queue.len() > 1 {
            _current = queue.last().unwrap();
        } else {
            _current = queue.first().unwrap();
        }
        let metadata = _current.metadata().clone();
        let user = format!("<@{}>", ctx.author().id.to_string());
        if let poise::Context::Application(context) = ctx {
            match context.interaction {
                ApplicationCommand(app) => {
                    app.edit_original_interaction_response(ctx.discord().http.clone(), |m| {
                        m.create_embed(|e| {
                            let title = metadata.title.as_ref().unwrap();
                            let url = metadata.source_url.as_ref().unwrap();
                            e.color(serenity::utils::Color::FADED_PURPLE);
                            e.description(format!("Queued [**{}**]({}) [{}]", title, url, user));
                            e
                        })
                    })
                    .await?;
                }
                Autocomplete(_) => todo!(),
            }
        }
    } else {
        send_message(ctx, "Could not join").await;
    }

    Ok(())
}

// Event to check if the bot is voice channel
// if no one in is voice channel leave and clear the current_queue
struct ChannelDurationNotifier {
    chan_id: ChannelId,
    guild_id: GuildId,
    http: Arc<Http>,
    ctx: SerenityContext,
}

#[async_trait]
impl VoiceEventHandler for ChannelDurationNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let mut members = 0;
        match self.chan_id.to_channel(&self.http).await.unwrap().guild() {
            Some(guild_channel) => {
                members = guild_channel.members(&self.ctx.cache).await.unwrap().len();
            }
            None => {
                println!("It's not in a guild!");
            }
        };
        if members == 1 {
            let manager = songbird::get(&self.ctx)
                .await
                .expect("Songbird Voice client placed in at initialisation.")
                .clone();
            let has_handler = manager.get(self.guild_id).is_some();
            if has_handler {
                if let Err(_e) = manager.remove(self.guild_id).await {
                    println!("left voice channel");
                } else {
                    println!("already left");
                }
            }
        }
        None
    }
}
