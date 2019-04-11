// look out this file to know what this bot do

use serenity::{
    model::{
        channel::Message, 
        event::PresenceUpdateEvent, 
        gateway::{ Ready, Game, Presence }, 
        user::OnlineStatus },
    prelude::{Context, EventHandler},
};

use crate::meltomos::*;
use crate::utils::*;

mod talk;
mod watch;
mod util;
mod art;
mod combat;

pub struct Handler;

impl EventHandler for Handler {
    
    // reaction for messages
    fn message(&self, _: Context, msg: Message) {
        if msg.author.bot { return; }
        // to direct message
        if msg.is_private() {
            if command_handle(&msg, &*msg.content) { return; }
            interactive_handle(&msg);
        // to public message
        } else {
            interactive_handle(&msg);
        }
        if command_handle_with_prefix(&msg) { return; }
    }

    // test
    fn presence_replace(&self, _: Context, presences: Vec<Presence>) {
        println!("presence replace caught:");
        for (i, presence) in presences.iter().enumerate() {
            println!("                {:?} | {:?}", i, presence);
        }
    }

    // reaction for status update
    // TODO add more (havnt decided what)
    fn presence_update(&self, _: Context, event: PresenceUpdateEvent) {
        let target_player = find_meltomo(&event.presence.user_id);
        if target_player.game_changed(&event.presence.game) {
            watch::stat_update(event.presence.game.as_ref(), &target_player)
        }
    }

    fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.shard.set_presence(Some(Game::playing("Fate/EXTRA")), OnlineStatus::Online);
    }
}

fn command_handle(msg: &Message, text: &str) -> bool {
    match text {
        "help" => talk::help(&msg),

        "watchme" => watch::watch(&msg.author),
        "unwatchme" => watch::unwatch(&msg.author),
        "status" => watch::status(&msg),
        "list" => watch::list(),

        "whoami" => talk::whois(&msg),

        "janken" => talk::command_battle(&msg),

        "e" => art::random(&msg),
        // temporal solution
        "save" => crate::meltomos::save(),

        _ => return false
    }
    true
}

fn command_handle_with_prefix(msg: &Message) -> bool {
    if let Some(text) = has_prefix(&*msg.content) {
        if !command_handle(&msg, text) {
            talk::dunno(&msg);
        }
        return true;
    }
    false
}

fn interactive_handle_core(msg: &Message, text: &str) -> bool {
    match text {
        "e" => combat::choose(&msg, combat::Difficulty::EASY),

        _ => false
    }
}

fn interactive_handle(msg: &Message) -> bool {
    if let Some(text) = has_prefix(&*msg.content) {
        return interactive_handle_core(msg, text);
    } else {
        return interactive_handle_core(msg, &*msg.content);
    }
}
