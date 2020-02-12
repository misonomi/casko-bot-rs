use serenity::{
    model::channel::Message,
    prelude::Context,
};

use super::util::{ talk_facade, react_facade };
use crate::meltomos;
use crate::meltomos::stat::TalkSequence;

pub fn start(c: &Context, msg: &Message) -> bool {
    if msg.is_private() { return false }
    meltomos::update_seq(&msg.author.id, TalkSequence::Vote);
    react_facade(c, msg, "✅");
    talk_facade(c, &msg.channel_id, "*help*");
    true
}
