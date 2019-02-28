use std::env;
use serenity::client::Client;
use serenity::prelude::EventHandler;

struct Handler;

impl EventHandler for Handler {}

pub fn main() {
    let token = env::var("CASKO_DISCORD_BOT_TOKEN").expect("set 'CASKO_DISCORD_BOT_TOKEN' to environment variable");

    let client = Client::new(&token, Handler).expect("failed to create client");
}
