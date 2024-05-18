use std::env;

use dotenv;
use poise::{
    builtins::register_globally,
    serenity_prelude::{
        GatewayIntents,
        ClientBuilder
    },

    Framework,
    FrameworkOptions,
    PrefixFrameworkOptions,
};

use nazrin::{get_cmds, NazrinData};
use songbird::SerenityInit;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Missing token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let framework = Framework::builder()
        .options(FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            commands: get_cmds(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                register_globally(ctx, &framework.options().commands).await?;
                Ok(NazrinData::new())
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;
    client.unwrap().start().await.unwrap();
}