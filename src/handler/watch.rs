use serenity::{
    model::{ gateway::Presence, channel::Message },
    prelude::Context,
};

use crate::meltomos;
use crate::meltomos::stat::BondType;
use super::util::{ talk_facade, dm_facade, minutes };

// change status of msg's author to watching and say so
pub fn watch(c: &Context, msg: &Message) -> bool {
    match meltomos::watch(&msg.author.id) {
        Ok(_) => talk_facade(c, &msg.channel_id, "Now I'm watching you!"),
        Err(_) => talk_facade(c, &msg.channel_id, "I'm already watching you."),
    }
    true
}

// change status of msg's author to unwatching and say so
pub fn unwatch(c: &Context, msg: &Message) -> bool {
    match meltomos::unwatch(&msg.author.id) {
        Ok(_) => talk_facade(c, &msg.channel_id, "I'm no longer watching you."),
        Err(_) => talk_facade(c, &msg.channel_id, "I'm not watching you."),
    }
    true
}

pub fn status(c: &Context, msg: &Message) -> bool {
    match meltomos::get_stat(&msg.author.id) {
        Some(BondType::Normal) => talk_facade(c, &msg.channel_id, "We are meltomo(pen pals), right? At least I think so."),
        Some(BondType::Watching) => talk_facade(c, &msg.channel_id, "I'm watching you."),
        Some(BondType::Admin) => talk_facade(c, &msg.channel_id, "You are administrator of this bot."),
        _ => talk_facade(c, &msg.channel_id, "I didn't know you, but now I know you."),
    }
    true
}

// capture a watching player's status change and dm
pub fn game_update(c: &Context, pres: Presence) -> bool {
    if meltomos::conjecture_game(&pres.user_id, pres.activity.as_ref()) { return false; }
    let user = &pres.user_id.to_user(c).expect("failed to get user data");
    let (old, time) = meltomos::exchange_game(&pres.user_id, pres.activity.clone());
    match (pres.activity, old) {
        (Some(new_game), Some(old_game)) => {
            dm_facade(c, user, &*format!("You started {} and thus quit {}, which had played for {} minutes.", new_game.name, old_game.name, minutes(time)));
        },
        (Some(new_game), None) => {
            dm_facade(c, user, &*format!("You started {}.", new_game.name));
        },
        (None, Some(old_game)) => {
            dm_facade(c, user, &*format!("You have played {} for {} minutes.", old_game.name, minutes(time)));
        },
        (None, None) => ()
    }
    true
}

// list up meltomo info
pub fn list(msg: &Message) -> bool {
    if meltomos::conjecture_stat(&msg.author.id, BondType::Admin) { return false; }
    meltomos::list();
    true
}

// save meltomo list to file
pub fn save(msg: &Message) -> bool {
    if meltomos::conjecture_stat(&msg.author.id, BondType::Admin) { return false; }
    meltomos::save();
    true
}
