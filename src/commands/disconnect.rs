use crate::{check, send_message, Context, Error};

/// Command to leave from voice channel
#[poise::command(slash_command, check = "check")]
pub async fn disconnect(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(ctx.discord())
        .await
        .expect("Songbird Voice client placed in at initialisation")
        .clone();

    manager.remove(guild_id).await?;
    send_message(ctx, ":skull_crossbones: Left Voice ").await;

    Ok(())
}
