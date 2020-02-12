// look out this file to know what this bot do

use serenity::{
    model::{
        channel::Message, 
        event::PresenceUpdateEvent, 
        gateway::{ Ready, Activity, Presence }, 
        user::OnlineStatus },
    prelude::{ Context, EventHandler },
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
    fn message(&self, c: Context, msg: Message) {
        if msg.author.bot { return; }
        // to direct message
        if msg.is_private() {
            meltomos::add_meltomo(&msg.author.id);
            handle_private(c, &msg)
        // to public message
        } else {
            handle_public(c, &msg)
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
    fn presence_update(&self, c: Context, event: PresenceUpdateEvent) {
        watch::game_update(&c, event.presence);
    }

    fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.shard.set_presence(Some(Activity::playing("Fate/EXTRA")), OnlineStatus::Online);
    }
}

fn command_handle(c: &Context, msg: &Message, text: &str) -> bool {
    match text {
        "help" => talk::help(c, msg),
        "combat_help" => talk::combat_help(c, msg),
        "vote_help" => talk::vote_help(c, msg),

        "watchme" => watch::watch(c, msg),
        "unwatchme" => watch::unwatch(c, msg),
        "status" => watch::status(c, msg),

        "janken" => combat::start(c, msg),

        "vote" => vote::start(c, msg),

        "freetalk" => freetalk::start(c, msg),
        
        "quit" => talk::quit(c, msg),

        "e" => art::random(c, msg),

        "whoami" => talk::whois(c, msg),
        "list" => watch::list(msg),
        "save" => watch::save(msg),

        _ => false
    }
}

fn interactive_handle(c: &Context, msg: &Message, text: &str) -> bool {
    match text {
        "e" | "easy" => combat::choose(c, msg, Difficulty::Easy),
        "n" | "normal" => combat::choose(c, msg, Difficulty::Normal),
        "h" | "hard" => combat::choose(c, msg, Difficulty::Hard),
        battle if util::HAND_PATTERN.is_match(battle) => combat::battle(c, msg),

        _ => freetalk::talk(c, msg),
    }
}

fn handle_private(c: Context, msg: &Message) {
    let handled = if let Some(text) = util::remove_prefix(&*msg.content) {
        command_handle(&c, msg, text) || interactive_handle(&c, msg, text)
    } else {
        command_handle(&c, msg, &*msg.content) || interactive_handle(&c, msg, &*msg.content)
    };
    if !handled { talk::dunno(&c, msg); }
}

fn handle_public(c: Context, msg: &Message) {
    let handled = if let Some(text) = util::remove_prefix(&*msg.content) {
        command_handle(&c, msg, text) || interactive_handle(&c, msg, text)
    } else if meltomos::is_talking(&msg.author.id) {
        interactive_handle(&c, msg, &*msg.content);
        true
    } else {
        true
    };
    if !handled { talk::dunno(&c, msg); }
}
