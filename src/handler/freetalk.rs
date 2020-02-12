use serenity::{
    model::channel::Message,
    prelude::Context,
};

use super::util::{ talk_facade, react_facade };
use crate::meltomos;
use crate::meltomos::stat::TalkSequence;

pub fn start(c: &Context, msg: &Message) -> bool {
    meltomos::update_seq(&msg.author.id, TalkSequence::FreeTalk);
    react_facade(c, msg, "ðŸ˜„");
    talk_facade(c, &msg.channel_id, "starting free talk.");
    true
}

pub fn talk(_: &Context, msg: &Message) -> bool {
    if !meltomos::conjecture_seq(&msg.author.id, TalkSequence::FreeTalk) { return false; }
    // TODO nlp!!!!!!!
    true
}