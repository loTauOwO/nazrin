use reqwest::Client;

use crate::commands::{common, music};

#[derive(Clone)]
pub struct NazrinData {
    http_client: Client,
}

impl NazrinData {
    pub fn new() -> NazrinData {
        NazrinData {
            http_client: Client::new(),
        }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, NazrinData, Error>;

pub mod commands {
    pub mod common;
    pub mod music;
}

pub fn get_cmds() -> Vec<poise::Command<NazrinData, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>> {
    vec![
        common::help(),
        common::ping(),
        common::avatar(),
        music::join(),
        music::play(),
    ]
}