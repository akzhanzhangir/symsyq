use crate::{check, send_message, Context, Error};

/// Command to pause song
#[poise::command(slash_command, check = "check")]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;
    let manager = songbird::get(ctx.discord()).await.unwrap();
    let call = manager.get(guild_id).unwrap();
    let handler = call.lock().await;
    let queue = handler.queue();
    if queue.pause().is_ok() {
        send_message(ctx, ":pause_button: Paused the player").await;
    }
    Ok(())
}
