use serenity::model::{ gateway::Presence, channel::Message };

use crate::meltomos;
use crate::meltomos::stat::BondType;
use super::util::{ talk_facade, dm_facade, minutes };

// change status of msg's author to watching and say so
pub fn watch(msg: &Message) {
    match meltomos::watch(&msg.author.id) {
        Ok(_) => talk_facade(&msg.channel_id, "Now I'm watching you!"),
        Err(_) => talk_facade(&msg.channel_id, "I'm already watching you."),
    }
}

// change status of msg's author to unwatching and say so
pub fn unwatch(msg: &Message) {
    match meltomos::unwatch(&msg.author.id) {
        Ok(_) => talk_facade(&msg.channel_id, "I'm no longer watching you."),
        Err(_) => talk_facade(&msg.channel_id, "I'm not watching you."),
    }
}

pub fn status(msg: &Message) {
    match meltomos::get_stat(&msg.author.id) {
        Some(BondType::Normal) => talk_facade(&msg.channel_id, "We are meltomo(pen pals), right? At least I think so."),
        Some(BondType::Watching) => talk_facade(&msg.channel_id, "I'm watching you."),
        Some(BondType::Admin) => talk_facade(&msg.channel_id, "You are administrator of this bot."),
        _ => talk_facade(&msg.channel_id, "I didn't know you, but now I know you."),
    }
}

// capture a watching player's status change and dm
pub fn game_update(pres: Presence) {
    if meltomos::conjecture_game(&pres.user_id, pres.game.as_ref()) { return; }
    let user = &pres.user_id.to_user().expect("failed to get user data");
    let (old, time) = meltomos::exchange_game(&pres.user_id, pres.game.clone());
    match (pres.game, old) {
        (Some(new_game), Some(old_game)) => {
            dm_facade(user, &*format!("You started {} and thus quit {}, which had played for {} minutes.", new_game.name, old_game.name, minutes(time)));
        },
        (Some(new_game), None) => {
            dm_facade(user, &*format!("You started {}.", new_game.name));
        },
        (None, Some(old_game)) => {
            dm_facade(user, &*format!("You have played {} for {} minutes.", old_game.name, minutes(time)));
        },
        (None, None) => ()
    }
}

// list up meltomo info
pub fn list(msg: &Message) {
    if meltomos::conjecture_stat(&msg.author.id, BondType::Admin) { return; }
    meltomos::list();
}

// save meltomo list to file
pub fn save(msg: &Message) {
    if meltomos::conjecture_stat(&msg.author.id, BondType::Admin) { return; }
    meltomos::save();
}
