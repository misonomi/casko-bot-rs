// look out this file to know what this bot do

use serenity::{
    model::{
        channel::Message, 
        event::PresenceUpdateEvent, 
        gateway::{ Ready, Game, Presence }, 
        user::OnlineStatus },
    prelude::{Context, EventHandler},
};

use crate::meltomos;

mod talk;
mod watch;
mod util;
mod art;
pub mod combat;
use combat::Difficulty;

pub struct Handler;

impl EventHandler for Handler {
    
    // reaction for messages
    fn message(&self, _: Context, msg: Message) {
        if msg.author.bot { return; }
        // to direct message
        if msg.is_private() {
            meltomos::add_meltomo(&msg.author.id);
            if command_handle(&msg, &*msg.content) { return; }
            if interactive_handle(&msg) { return; }
        // to public message
        } else {
            if interactive_handle(&msg) { return; }
        }
        command_handle_with_prefix(&msg);
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
        let mut contacts_guarded = meltomos::get_lock();
        if let Some(mut target_player) = meltomos::find_meltomo(&event.presence.user_id, &mut contacts_guarded) {
            if target_player.game_changed(event.presence.game.as_ref()) {
                watch::stat_update(event.presence.game.as_ref(), &mut target_player);
            }
        }
    }

    fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.shard.set_presence(Some(Game::playing("Fate/EXTRA")), OnlineStatus::Online);
    }
}

fn command_handle(msg: &Message, text: &str) -> bool {
    match text {
        "help" => talk::help(msg),

        "watchme" => watch::watch(msg),
        "unwatchme" => watch::unwatch(msg),
        "status" => watch::status(msg),
        "list" => watch::list(),

        "whoami" => talk::whois(msg),

        "janken" => talk::command_battle(msg),

        "e" => art::random(msg),
        // temporal solution
        "save" => crate::meltomos::save(),

        _ => return false
    }
    true
}

fn command_handle_with_prefix(msg: &Message) -> bool {
    if let Some(text) = util::remove_prefix(&*msg.content) {
        meltomos::add_meltomo(&msg.author.id);
        if !command_handle(&msg, text) {
            talk::dunno(msg);
        }
        true
    } else {
        false
    }
}

fn interactive_handle_core(msg: &Message, text: &str) -> bool {
    match text {
        "e" => combat::choose(msg, Difficulty::Easy),
        "easy" => combat::choose(msg, Difficulty::Easy),
        "n" => combat::choose(msg, Difficulty::Normal),
        "normal" => combat::choose(msg, Difficulty::Normal),
        "h" => combat::choose(msg, Difficulty::Hard),
        "hard" => combat::choose(msg, Difficulty::Hard),

        _ => false
    }
}

fn interactive_handle(msg: &Message) -> bool {
    if let Some(text) = util::remove_prefix(&*msg.content) {
        return interactive_handle_core(msg, text);
    } else {
        return interactive_handle_core(msg, &*msg.content);
    }
}
