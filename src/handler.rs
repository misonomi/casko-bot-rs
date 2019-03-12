// look out this file to know what this bot do

use serenity::{
    model::{channel::Message, event::PresenceUpdateEvent },
    prelude::{Context, EventHandler},
};

use crate::watchees::*;

mod talk;
mod watch;
mod util;

pub struct Handler;

impl EventHandler for Handler {
    
    // reaction for messages
    fn message(&self, _: Context, msg: Message) {
        if msg.is_private() {
            match &*msg.content {
                "help" => talk::help(&msg),
                "watchme" => watch::watch(&msg.author),
                "unwatchme" => watch::unwatch(&msg.author),
                "status" => talk::status(&msg),
                "list" => watch::list(),
                _ => ()
            }
        }
    }

    // reaction for status update
    fn presence_update(&self, _: Context, event: PresenceUpdateEvent) {
        match game_changed(&event.presence.user_id, &event.presence.game) {
            Ok(true) => watch::stat_update(&event.presence),
            Ok(false) => (),
            Err(_) => ()
        }
    }
}
