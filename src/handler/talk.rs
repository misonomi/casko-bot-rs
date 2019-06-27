use serenity::model::{ channel::Message };

use super::util::{ dm_facade, talk_facade, react_facade };
use crate::meltomos;
use crate::meltomos::stat::TalkSequence;

pub fn help(msg: &Message) -> bool{
    react_facade(msg, "✅");
    dm_facade(&msg.author, "*help*\n*help*");
    true
}

pub fn combat_help(msg: &Message) -> bool {
    react_facade(msg, "✅");
    talk_facade(&msg.channel_id, "*help*");
    true
}

pub fn vote_help(msg: &Message) -> bool {
    react_facade(msg, "✅");
    talk_facade(&msg.channel_id, "*help*");
    true
}

pub fn quit(msg: &Message) -> bool {
    meltomos::update_seq(&msg.author.id, TalkSequence::None);
    react_facade(msg, "✅");
    talk_facade(&msg.channel_id, "exitting all conversation.");
    true
}

// TODO delete when its not needed
pub fn whois(msg: &Message) -> bool {
    react_facade(msg, "✅");
    dm_facade(&msg.author, &*format!("your ID is {}", msg.author.id));
    true
}

pub fn dunno(msg: &Message) {
    react_facade(msg, "🤔");
    talk_facade(&msg.channel_id, "unknown command. say '^^~ help' to get help dm");
}
