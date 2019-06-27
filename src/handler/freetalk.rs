use serenity::model::channel::Message;

use super::util::{ talk_facade, react_facade };
use crate::meltomos;
use crate::meltomos::stat::TalkSequence;

pub fn start(msg: &Message) -> bool {
    meltomos::update_seq(&msg.author.id, TalkSequence::FreeTalk);
    react_facade(msg, "ðŸ˜„");
    talk_facade(&msg.channel_id, "starting free talk.");
    true
}

pub fn talk(msg: &Message) -> bool {
    if !meltomos::conjecture_seq(&msg.author.id, TalkSequence::FreeTalk) { return false; }
    // TODO nlp!!!!!!!
    true
}