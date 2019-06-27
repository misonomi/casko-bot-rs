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
mod art;
mod freetalk;
mod vote;
pub mod combat;
mod util;
use combat::Difficulty;

pub struct Handler;

impl EventHandler for Handler {
    
    // reaction for messages
    fn message(&self, _: Context, msg: Message) {
        if msg.author.bot { return; }
        // to direct message
        if msg.is_private() {
            meltomos::add_meltomo(&msg.author.id);
            handle_private(&msg)
        // to public message
        } else {
            handle_public(&msg)
        }
    }

    // test remove if not working
    fn presence_replace(&self, _: Context, presences: Vec<Presence>) {
        println!("presence replace caught:");
        for (i, presence) in presences.iter().enumerate() {
            println!("                {:?} | {:?}", i, presence);
        }
    }

    // reaction for status update
    // TODO add more (havnt decided what)
    fn presence_update(&self, _: Context, event: PresenceUpdateEvent) {
        watch::game_update(event.presence);
    }

    fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.shard.set_presence(Some(Game::playing("Fate/EXTRA")), OnlineStatus::Online);
    }
}

fn command_handle(msg: &Message, text: &str) -> bool {
    match text {
        "help" => talk::help(msg),
        "combat_help" => talk::combat_help(msg),
        "vote_help" => talk::vote_help(msg),

        "watchme" => watch::watch(msg),
        "unwatchme" => watch::unwatch(msg),
        "status" => watch::status(msg),

        "janken" => combat::start(msg),

        "vote" => vote::start(msg),

        "freetalk" => freetalk::start(msg),
        
        "quit" => talk::quit(msg),

        "e" => art::random(msg),

        "whoami" => talk::whois(msg),
        "list" => watch::list(msg),
        "save" => watch::save(msg),

        _ => false
    }
}

fn interactive_handle(msg: &Message, text: &str) -> bool {
    match text {
        "e" | "easy" => combat::choose(msg, Difficulty::Easy),
        "n" | "normal" => combat::choose(msg, Difficulty::Normal),
        "h" | "hard" => combat::choose(msg, Difficulty::Hard),
        battle if util::HAND_PATTERN.is_match(battle) => combat::battle(msg),

        _ => freetalk::talk(msg),
    }
}

fn handle_private(msg: &Message) {
    let handled = if let Some(text) = util::remove_prefix(&*msg.content) {
        command_handle(msg, text) || interactive_handle(msg, text)
    } else {
        command_handle(msg, &*msg.content) || interactive_handle(msg, &*msg.content)
    };
    if !handled { talk::dunno(msg); }
}

fn handle_public(msg: &Message) {
    let handled = if let Some(text) = util::remove_prefix(&*msg.content) {
        command_handle(msg, text) || interactive_handle(msg, text)
    } else if meltomos::is_talking(&msg.author.id) {
        interactive_handle(msg, &*msg.content);
        true
    } else {
        true
    };
    if !handled { talk::dunno(msg); }
}
