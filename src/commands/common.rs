use poise::{command, serenity_prelude::{Mentionable, User}};

use crate::{Context, Error};

#[poise::command(prefix_command)]
pub async fn ping(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}

#[command(prefix_command)]
async fn halo(
    ctx: Context<'_>,
    #[description = "selected user"]
    user: Option<User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("hello {}", u.mention());
    ctx.say(response).await?;
    Ok(())
}

#[command(prefix_command)]
async fn help(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.say("go help yourselft").await?;
    Ok(())
}