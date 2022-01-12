use crate::{check, send_error, send_message, Context, Error};
use std::time::Duration;

/// Command to jump some time in a song
#[poise::command(slash_command, check = "check")]
pub async fn seek(
    ctx: Context<'_>,
    #[description = "Time position in format MM:SS"] position: String,
) -> Result<(), Error> {
    let splitted: Vec<&str> = position.as_str().split(':').collect();
    let err_message: &str = "Couldn't parse time";
    let min = match splitted[0].parse::<u64>() {
        Ok(min) => min,
        Err(_) => {
            send_error(ctx, err_message).await;
            return Ok(());
        }
    };
    let sec = match splitted[1].parse::<u64>() {
        Ok(sec) => sec,
        Err(_) => {
            send_error(ctx, err_message).await;
            return Ok(());
        }
    };
    let seek_time = match from_min_sec(min, sec) {
        Some(time) => time,
        None => {
            send_error(ctx, err_message).await;
            return Ok(());
        }
    };

    let guild_id = ctx.guild().unwrap().id;
    let manager = songbird::get(ctx.discord()).await.unwrap();
    let call = manager.get(guild_id).unwrap();
    let handler = call.lock().await;
    let track = handler.queue().current().unwrap();
    drop(handler);
    track.seek_time(seek_time).unwrap();
    send_message(ctx, format!("Seeked the track to {}", position).as_str()).await;

    Ok(())
}

fn from_min_sec(min: u64, sec: u64) -> Option<Duration> {
    if sec >= 60 {
        return None;
    }
    let secs = min * 60 + sec;
    Some(Duration::from_secs(secs))
}
