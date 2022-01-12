use songbird::tracks::LoopState;

use crate::{check, send_message, Context, Error};
/// Command to repeat song infinitly
#[poise::command(slash_command, rename = "loop", check = "check")]
pub async fn repeat(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;
    let manager = songbird::get(ctx.discord()).await.unwrap();
    let call = manager.get(guild_id).unwrap();
    let handler = call.lock().await;

    let track = handler
        .queue()
        .current()
        .expect("Failed to fetch handle for current track");

    drop(handler);

    if track.get_info().await?.loops == LoopState::Infinite {
        if track.disable_loop().is_ok() {
            send_message(ctx, ":repeat:  Disabled loop :negative_squared_cross_mark:").await;
        }
    } else if track.enable_loop().is_ok() {
        send_message(ctx, ":repeat:  Enabled loop :white_check_mark:").await;
    }

    Ok(())
}
