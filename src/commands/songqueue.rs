use crate::{check, get_human_readable_timestamp, Context, Error};
use serenity::model::interactions::message_component::ButtonStyle;
use songbird::tracks::TrackHandle;

use std::cmp::{max, min};

const PAGE_SIZE: usize = 10;

/// Command to show queue of songs
#[poise::command(slash_command, rename = "queue", check = "check")]
pub async fn songqueue(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild().unwrap().id;
    let manager = songbird::get(ctx.discord()).await.unwrap();
    let call = manager.get(guild_id).unwrap();
    let handler = call.lock().await;
    let queue = handler.queue();
    let tracks;
    tracks = queue.current_queue();
    drop(handler);

    let uuid_first = "first";
    let uuid_back = "back";
    let uuid_next = "next";
    let uuid_last = "last";

    ctx.send(|m| {
        m.content(build_queue(&tracks, 0)).components(|c| {
            c.create_action_row(|ar| {
                ar.create_button(|b| {
                    b.style(ButtonStyle::Secondary)
                        .label(uuid_first)
                        .custom_id(uuid_first)
                })
                .create_button(|b| {
                    b.style(ButtonStyle::Secondary)
                        .label(uuid_back)
                        .custom_id(uuid_back)
                })
                .create_button(|b| {
                    b.style(ButtonStyle::Secondary)
                        .label(uuid_next)
                        .custom_id(uuid_next)
                })
                .create_button(|b| {
                    b.style(ButtonStyle::Secondary)
                        .label(uuid_last)
                        .custom_id(uuid_last)
                })
            })
        })
    })
    .await?;

    let num_pages = ((tracks.len() as f64 - 1.0) / PAGE_SIZE as f64).ceil() as usize;
    let pages = max(1, num_pages);

    let mut page: usize = 0;
    while let Some(mci) =
        serenity::collector::component_interaction_collector::CollectComponentInteraction::new(
            ctx.discord(),
        )
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| {
            mci.data.custom_id == uuid_first
                || mci.data.custom_id == uuid_back
                || mci.data.custom_id == uuid_next
                || mci.data.custom_id == uuid_last
        })
        .await
    {
        page = match mci.data.custom_id.as_str() {
            "first" => 0,
            "back" => min(page.saturating_sub(1), pages - 1),
            "next" => min(page + 1, pages - 1),
            "last" => pages - 1,
            _ => continue,
        };

        let mut msg = mci.message.clone();
        msg.edit(ctx.discord(), |m| m.content(build_queue(&tracks, page)))
            .await?;

        mci.create_interaction_response(ctx.discord(), |ir| {
            ir.kind(serenity::model::interactions::InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;
    }

    Ok(())
}

fn build_queue(tracks: &[TrackHandle], page: usize) -> String {
    let start_idx = PAGE_SIZE * page;
    let queue: Vec<&TrackHandle> = tracks.iter().skip(start_idx).take(PAGE_SIZE).collect();

    let mut description = String::new();
    description.push_str(&"```ml\n".to_string());

    for (i, t) in queue.iter().enumerate() {
        let title = t.metadata().title.as_ref().unwrap();
        let answer;
        if title.len() > 36 {
            answer = &title[..36];
        } else {
            answer = title;
        }
        let duration = get_human_readable_timestamp(t.metadata().duration.unwrap());

        description.push_str(&format!(
            "{:>2}) {:<36} {:>}\n",
            i + start_idx + 1,
            answer,
            duration
        ));
    }

    description.push_str(&"```".to_string());

    description
}
