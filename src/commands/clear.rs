use crate::{check, Context, Error};

/// Command to clear the queue
#[poise::command(slash_command, check = "check")]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;
    let manager = songbird::get(ctx.discord()).await.unwrap();
    let call = manager.get(guild_id).unwrap();
    let handler = call.lock().await;
    let len = handler.queue().current_queue().capacity();
    if len > 1 {
        handler.queue().modify_queue(|v| {
            v.drain(1..);
        });
    }
    Ok(())
}
