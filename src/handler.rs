// look out this file to know what this bot do

use serenity::{
    model::{channel::Message, gateway::Ready, event::PresenceUpdateEvent },
    prelude::{Context, EventHandler},
};

pub mod talk;
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
            watch::watch(&msg.author);
        }
        if msg.content == format!("{}unwatchme", prefix) {
            watch::unwatch(&msg.author);
        }
        if msg.content == format!("{}status", prefix) {
            talk::status(&msg);
        }
        if msg.content == format!("{}list", prefix) {
            watch::list();
        }
    }

    // reaction for user login
    fn ready(&self, _: Context, ready: Ready) {
        //talk::welcome(&ready.user);
    }

    // reaction for status update
    fn presence_update(&self, _: Context, event: PresenceUpdateEvent) {
        if event.presence.user.is_some() {
            watch::stat_update(&event.presence);
        }
    }
}