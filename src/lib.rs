use crate::commands::common;

pub struct NazrinData {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, NazrinData, Error>;

pub mod commands {
    pub mod common;
}

pub fn get_cmds() -> Vec<poise::Command<NazrinData, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>> {
    vec![
        common::ping(),
    ]
}