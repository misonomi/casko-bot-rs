use serenity::model::{ channel::Message };

use super::util::{ dm_facade, talk_facade, react_facade };
use crate::meltomos;
use crate::meltomos::stat::TalkSequence;

pub fn help(msg: &Message) -> bool{
    react_facade(msg, "✅");
    dm_facade(&msg.author, "*help*");
    true
}

pub fn combat_help(msg: &Message) -> bool {
    react_facade(msg, "✅");
    talk_facade(&msg.channel_id, "*help*");
    true
}

pub fn command_battle(msg: &Message) -> bool {
    // TODO change bahavior according to talk sequence
    meltomos::update_seq(&msg.author.id, TalkSequence::ChooseDiffic);
    react_facade(msg, "🔥");
    talk_facade(&msg.channel_id, "I accept your challenge. choose difficulty.");
    talk_facade(&msg.channel_id, "say e(easy) / n(normal) / h(hard)");
    true
}

pub fn free_talk(msg: &Message) -> bool {
    meltomos::update_seq(&msg.author.id, TalkSequence::FreeTalk);
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
