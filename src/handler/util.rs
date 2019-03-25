use std::time::Instant;

use serenity::model::{ id::ChannelId, user::User, channel::{ Message, ReactionType } };

pub fn dm_facade(user: &User, mes: &str) {
    if let Err(cause) = user.dm(|m| m.content(mes)) {
        println!("Error when direct messaging user: {:?}", cause);
    }
}

pub fn talk_facade(channel: &ChannelId, mes: &str) {
    if let Err(cause) = channel.say(mes) {
        println!("Error when talking: {:?}", cause);
    }
}

pub fn react_facade(mes: &Message, unicode: &str) {
    if let Err(cause) = mes.react(ReactionType::from(unicode)) {
        println!("Error when reacting: {:?}", cause);
    }
}

pub fn minutes(from: &Instant) -> u64 {
    from.elapsed().as_secs() / 60
}
