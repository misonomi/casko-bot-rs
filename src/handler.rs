// look out this file to know what this bot do

use serenity::{
    model::{channel::Message, gateway::Ready, event::PresenceUpdateEvent },
    prelude::{Context, EventHandler},
};

mod talk;
mod player_stat;

pub struct Handler;

impl EventHandler for Handler {
    // reaction for messages
    fn message(&self, _: Context, msg: Message) {
        let prefix = "^^~";

        // send dm about the help to use this bot
        if msg.content == format!("{}help", prefix) {
            talk::help(&msg);
        }
        // change author's "watching status" to on
        if msg.content == format!("{}watchme", prefix) {
            player_stat::watch(&msg);
        }
        // change author's "watching status" to off
        if msg.content == format!("{}unwatchme", prefix) {
            player_stat::unwatch(&msg);
        }
        // tell the "watching status" for the user
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
            talk::stat_update(&event.presence);
        }
    }
}