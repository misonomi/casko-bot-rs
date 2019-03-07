use std::env;
use serenity::client::Client;

mod handler;
mod watchees;

pub fn main() {
    let token = env::var("CASKO_DISCORD_BOT_TOKEN").expect("set 'CASKO_DISCORD_BOT_TOKEN' to environment variable");

    // behavior is set to handler
    let mut client = Client::new(&token, handler::Handler).expect("failed to create client");

    if let Err(cause) = client.start() {
        println!("failed to start client. cause : {}", cause);
    }
}
