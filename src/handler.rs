// look out this file to know what this bot do

use serenity::{
    model::{channel::Message, event::PresenceUpdateEvent },
    prelude::{Context, EventHandler},
};

use crate::watchees::*;

mod talk;
mod watch;
mod util;
mod art;

pub struct Handler;

impl EventHandler for Handler {
    
    // reaction for messages
    fn message(&self, _: Context, msg: Message) {
        if msg.is_private() {
            match &*msg.content {
                "help" => talk::help(&msg),
                "watchme" => watch::watch(&msg.author),
                "unwatchme" => watch::unwatch(&msg.author),
                "status" => watch::status(&msg),
                "list" => watch::list(),
                "whoami" => talk::whois(&msg),
                "e" => art::random(&msg),
                _ => ()
            }
        }
    }

    // reaction for status update
    // TODO add more (havnt decided what)
    fn presence_update(&self, _: Context, event: PresenceUpdateEvent) {
        let target_player = find_watchee(&event.presence.user_id);
        if target_player.game_changed(&event.presence.game) {
            watch::stat_update(&event.presence.game, &target_player)
        }
    }
}
