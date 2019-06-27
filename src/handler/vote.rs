use serenity::model::channel::Message;

use super::util::{ talk_facade, react_facade };
use crate::meltomos;
use crate::meltomos::stat::TalkSequence;

pub fn start(msg: &Message) -> bool {
    if msg.is_private() { return false }
    meltomos::update_seq(&msg.author.id, TalkSequence::Vote);
    react_facade(msg, "✅");
    talk_facade(&msg.channel_id, "*help*");
    true
}
