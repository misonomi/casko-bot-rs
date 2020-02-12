use serenity::{
    model::{ channel::Message },
    prelude::Context,
};

use super::util::{ dm_facade, talk_facade, react_facade };
use crate::meltomos;
use crate::meltomos::stat::TalkSequence;

pub fn help(c: &Context, msg: &Message) -> bool{
    react_facade(c, msg, "✅");
    dm_facade(c, &msg.author, "*help*\n*help*");
    true
}

pub fn combat_help(c: &Context, msg: &Message) -> bool {
    react_facade(c, msg, "✅");
    talk_facade(c, &msg.channel_id, "*help*");
    true
}

pub fn vote_help(c: &Context, msg: &Message) -> bool {
    react_facade(c, msg, "✅");
    talk_facade(c, &msg.channel_id, "*help*");
    true
}

pub fn quit(c: &Context, msg: &Message) -> bool {
    meltomos::update_seq(&msg.author.id, TalkSequence::None);
    react_facade(c, msg, "✅");
    talk_facade(c, &msg.channel_id, "exitting all conversation.");
    true
}

// TODO delete when its not needed
pub fn whois(c: &Context, msg: &Message) -> bool {
    react_facade(c, msg, "✅");
    dm_facade(c, &msg.author, &*format!("your ID is {}", msg.author.id));
    true
}

pub fn dunno(c: &Context, msg: &Message) {
    react_facade(c, msg, "🤔");
    talk_facade(c, &msg.channel_id, "unknown command. say '^^~ help' to get help dm");
}
