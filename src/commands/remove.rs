use crate::{check, send_error, send_message, Context, Error};

/// Command to remove a song from queue
#[poise::command(slash_command, check = "check")]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Song queue position"] position: usize,
) -> Result<(), Error> {
    if position == 1 || position == 0 {
        send_error(ctx, "Can't remove the currently playing track").await;
        return Ok(());
    }
    let guild_id = ctx.guild().unwrap().id;
    let manager = songbird::get(ctx.discord()).await.unwrap();
    let call = manager.get(guild_id).unwrap();
    let handler = call.lock().await;
    let queue = handler.queue();

    let len = queue.current_queue().capacity();
    if position > len {
        send_error(ctx, "Queue values out of reach").await;
        return Ok(());
    }
    let _ = queue.pause();
    queue.modify_queue(|q| {
        q.remove(position - 1);
    });
    if queue.resume().is_ok() {
        send_message(ctx, ":cd: Track removed ✘").await;
    }
    Ok(())
}
