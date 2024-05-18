use std::{collections::HashMap, hash::RandomState};

use poise::serenity_prelude::{async_trait, UserId, VoiceState};
use songbird::{input::YoutubeDl, Event, EventContext, EventHandler, TrackEvent};

use crate::{Context, Error};

struct TrackErrorNotifier;

#[async_trait]
impl EventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}
#[poise::command(
    prefix_command,
    guild_only,
    category = "Music",
)]
pub async fn join(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let channel_id = (&guild.voice_states as &HashMap<UserId, VoiceState, RandomState>)
                .get(&ctx.author().id)
                .and_then(|vs| vs.channel_id); 
        (guild.id, channel_id)
    };
    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.say("You're not in voice channel").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.serenity_context())
    .await
    .expect("Songbird client placed in a voice channel")
    .clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier)
    }
    Ok(())
}

#[poise::command(
    prefix_command,
    guild_only,
    category = "Music",
)]
pub async fn play(
    ctx: Context<'_>,
    url: String
) -> Result<(), Error> {
    if !url.starts_with("http") {
        ctx.say("url must http").await?;
        return Ok(())
    }

    let manager = songbird::get(ctx.serenity_context())
    .await
    .expect("Songbird client placed in a voice channel")
    .clone();

    if let Some(handler_lock) = manager.get(ctx.guild_id().unwrap()) {
        let mut handler = handler_lock.lock().await;
        let src = YoutubeDl::new(ctx.data().http_client.clone(), url);
        handler.play_input(src.into());
    } else {
        ctx.say("Not in voice channel").await?;
    }
    Ok(())
}