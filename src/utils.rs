use crate::{Context, Error};

pub const EMPTY_QUEUE: &str = "The queue is empty :( \n use /play to add songs to queue";
pub const BOT_NOT_IN_VOCIE: &str = "The bot is not in voice channel :/";
pub const USER_NOT_IN_VOCIE: &str = "You are not in voice channel";
pub const ALREADY_PLAYING: &str = "The bot already running in another channel";

pub fn get_human_readable_timestamp(duration: std::time::Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = duration.as_secs() / 3600;

    if hours < 1 {
        format!("{}:{:02}", minutes, seconds)
    } else {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

// I used to send same type of message and i just put it in different function
pub async fn send_message(ctx: Context<'_>, message: &str) {
    ctx.send(|m| {
        m.embed(|e| {
            e.color(serenity::utils::Color::FADED_PURPLE);
            e.description(message);
            e
        })
    })
    .await
    .expect("Could not send message");
}

pub async fn send_error(ctx: Context<'_>, message: &str) {
    ctx.send(|m| {
        m.embed(|e| {
            e.color(serenity::utils::Color::RED);
            e.description(message);
            e
        })
    })
    .await
    .expect("Could not send message");
}

pub async fn check(ctx: Context<'_>) -> Result<bool, Error> {
    let guild_id = ctx.guild().unwrap().id;
    let manager = songbird::get(ctx.discord()).await.unwrap().clone();

    let call = match manager.get(guild_id) {
        Some(call) => call,
        None => {
            send_error(ctx, BOT_NOT_IN_VOCIE).await;
            return Ok(false);
        }
    };

    let handler = call.lock().await;
    let user_channel = match ctx
        .guild()
        .unwrap()
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id)
    {
        Some(user_channel) => user_channel,
        None => {
            send_error(ctx, USER_NOT_IN_VOCIE).await;
            return Ok(false);
        }
    };

    if handler.current_channel().unwrap().to_string() != user_channel.to_string() {
        send_error(ctx, ALREADY_PLAYING).await;
        return Ok(false);
    }

    if handler.queue().is_empty() {
        send_error(ctx, EMPTY_QUEUE).await;
        return Ok(false);
    }
    Ok(true)
}

/*
pub async fn get_queue(ctx: Context<'_>) -> &songbird::tracks::TrackQueue {
    let guild_id = ;
    let h = songbird::Songbird::get(ctx.guild().unwrap().id).unwrap().lock().await;
    let queue = h.queue();
    queue
}
*/
