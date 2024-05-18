use poise::{command, samples::HelpConfiguration, serenity_prelude::{CreateEmbed, CreateEmbedAuthor, User}, CreateReply};

use crate::{Context, Error};

#[poise::command(
    prefix_command,
    category = "Common",
)]
pub async fn ping(
    ctx: Context<'_>,
    #[flag]
    #[description = "Show detailed ping"]
    ws: bool,
) -> Result<(), Error> {
    let create_time = ctx.created_at();
    let repl = ctx.say(":ping_pong: | Ping....").await?;
    let msg = repl.message().await?;
    let response_time = msg.timestamp;
    let response_time = response_time.timestamp_millis() - create_time.timestamp_millis();

    let to_send;

    if ws {
        let create_time = ctx.ping().await;
        let create_time = create_time.as_millis();
        to_send = format!("\
:ping_pong: | Pong!
> Websocket `{create_time}ms`
> API `{response_time}ms`
        ", );
    } else {
        to_send = format!(":ping_pong: | Pong! `{response_time}ms`")
    }

    repl.edit(ctx, CreateReply {
        content: Some(to_send),
        ..Default::default()
    }).await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    category = "Common",
)]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "User to get the avatar"]
    user: Option<User>,
    #[flag]
    #[rename = "static"]
    #[description = "the returned avatar must static or not"]
    is_static: bool,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or(ctx.author());
    let avatar;
    if is_static {
        avatar = user.static_face();
    } else {
        avatar = user.face();
    }
    let embed = CreateEmbed::new()
        .color((255, 255, 0))
        .author(CreateEmbedAuthor::new(&user.name).icon_url(&avatar))
        .image(avatar);
    ctx.send(CreateReply { 
        embeds: vec![embed],
        ..Default::default()
    }).await?;
    Ok(())
}

#[command(
    prefix_command,
    category = "Common",
)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    command: Option<String>
) -> Result<(), Error> {
    let config = HelpConfiguration {
        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}