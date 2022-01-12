use crate::{check, get_human_readable_timestamp, Context, Error};

use serenity::builder::CreateEmbedFooter;

/// Get information about current song
#[poise::command(slash_command, check = "check")]
pub async fn song(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;
    let manager = songbird::get(ctx.discord()).await.unwrap();
    let call = manager.get(guild_id).unwrap();
    let handler = call.lock().await;
    let queue = handler.queue();
    send_song(ctx, queue.current().unwrap()).await?;
    drop(handler);

    Ok(())
}

async fn send_song(ctx: Context<'_>, current: songbird::tracks::TrackHandle) -> Result<(), Error> {
    let metadata = current.metadata().clone();
    let position = current.get_info().await?.position;
    let duration = metadata.duration.unwrap();
    let thumbnail = metadata.thumbnail.as_ref().unwrap();
    ctx.send(|m| {
        m.embed(|e| {
            let title = metadata.title.as_ref().unwrap();
            let url = metadata.source_url.as_ref().unwrap();

            e.title(title);
            e.url(url);
            e.thumbnail(thumbnail);
            e.color(serenity::utils::Color::FADED_PURPLE);

            let bar = build_player(position.as_millis(), duration.as_millis());
            e.description(bar);

            let mut footer = CreateEmbedFooter::default();
            let position_human = get_human_readable_timestamp(position);
            let duration_human = get_human_readable_timestamp(duration);

            footer.text(format!("{} / {}", position_human, duration_human));
            e.set_footer(footer)
        })
    })
    .await?;
    Ok(())
}

fn build_player(position: u128, duration: u128) -> String {
    let part = duration / 20_u128;
    let pos = (position / part) as usize;
    let line = '▬';
    let ball = '🔵';
    let mut bar = String::new();

    for n in 0..20 {
        if n != pos {
            bar.push(line);
        } else {
            bar.push(ball);
        }
    }

    bar
}
