use serenity::model::{ gateway::Game, channel::Message };

use crate::meltomos;
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
    match meltomos::has_meltomo(&msg.author.id) {
        Some(_) => talk_facade(&msg.channel_id, "I'm watching you."),
        None => talk_facade(&msg.channel_id, "I'm not watching you."),
    }
}

// capture a watching player's status change and dm
pub fn stat_update(game: Option<&Game>, player: &mut meltomos::meltomo::Meltomo) {
    let user = &player.to_user().expect("failed to get user data");
    match (game, player.game.as_ref()) {
        (Some(new_game), Some(old_game)) => {
            dm_facade(&user, &*format!("You started {} and thus quit {}, which had played for {} minutes.", new_game.name, old_game.name, minutes(&player.last_update)));
            player.update_game(Some(new_game.clone()));
        },
        (Some(new_game), None) => {
            dm_facade(&user, &*format!("You started {}.", new_game.name));
            player.update_game(Some(new_game.clone()));
        },
        (None, Some(old_game)) => {
            dm_facade(&user, &*format!("You have played {} for {} minutes.", old_game.name, minutes(&player.last_update)));
            player.update_game(None);
        },
        (None, None) => ()
    }
}

// TODO remove when release 
// list up meltomo info
pub fn list() {
    for (i, meltomo) in meltomos::get_lock().iter().enumerate() {
        println!("meltomos No.{}| id:{:?}, status:{:?}, sequence:{:?}, game:{:?}, timestamp:{:?}", 
                i, meltomo.id, meltomo.stat, meltomo.seq, meltomo.game, meltomo.last_update);
    }
}
