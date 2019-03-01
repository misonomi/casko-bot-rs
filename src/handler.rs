// look out this file to know what this bot do

use serenity::{
    model::{channel::Message, gateway::Ready, event::PresenceUpdateEvent },
    prelude::{Context, EventHandler},
};

mod talk;
mod watch;

pub struct Handler;

impl EventHandler for Handler {
    
    // reaction for messages
    fn message(&self, _: Context, msg: Message) {
        let prefix = "^^~";

        if msg.content == format!("{}help", prefix) {
            talk::help(&msg);
        }
        if msg.content == format!("{}watchme", prefix) {
            watch::watch(&msg);
        }
        if msg.content == format!("{}unwatchme", prefix) {
            watch::unwatch(&msg);
        }
        if msg.content == format!("{}status", prefix) {
            talk::status(&msg);
        }
    }

    // reaction for user login
    fn ready(&self, _: Context, ready: Ready) {
    }

    // reaction for status update
    fn presence_update(&self, _: Context, event: PresenceUpdateEvent) {
        if event.presence.user.is_some() {
            watch::stat_update(&event.presence);
        }
    }
}