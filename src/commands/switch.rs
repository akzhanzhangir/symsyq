use crate::{check, send_error, send_message, Context, Error};
use songbird::tracks::Queued;

use std::collections::VecDeque;

/// Command to swap song
#[poise::command(slash_command, rename = "swap", check = "check")]
pub async fn switch(
    ctx: Context<'_>,
    #[description = "First queue position"] first: usize,
    #[description = "Second queue position"] second: usize,
) -> Result<(), Error> {
    if first == 1 || second == 1 || first == 0 || second == 0 {
        send_error(ctx, "Can't swap the currently playing track").await;
        return Ok(());
    }
    let guild_id = ctx.guild().unwrap().id;
    let manager = songbird::get(ctx.discord()).await.unwrap();
    let call = manager.get(guild_id).unwrap();
    let handler = call.lock().await;
    let queue = handler.queue();

    let len = queue.current_queue().capacity();
    if first > len || second > len {
        send_error(ctx, "Queue values out of reach").await;
        return Ok(());
    }
    let swapped = |q: &mut VecDeque<Queued>| {
        q.swap(first - 1, second - 1);
    };
    queue.modify_queue(swapped);
    send_message(ctx, ":cd: Tracks swapped").await;
    Ok(())
}
