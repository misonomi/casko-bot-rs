use serenity::model::{ channel::Message };

use crate::watchees::*;
use super::util::{ dm_facade, talk_facade };

pub fn help(msg: &Message) {
    dm_facade(&msg.author, "*help*");
    talk_facade(&msg.channel_id, "sent help to you by direct message");
}

pub fn status(msg: &Message) {
    talk_facade(&msg.channel_id, match has_watchee(&msg.author.id) {
        Some(_) => "I'm watching you.",
        None => "I'm not watching you."
    });
}
