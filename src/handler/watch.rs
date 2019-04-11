use serenity::model::{ gateway::Game, user::User, channel::Message };

use crate::watchees::{ add_watchee, get_lock, remove_watchee, has_watchee, update_game, watchee::Watchee };
use super::util::{ talk_facade, dm_facade, minutes };

// change status of msg's author to watching and say so
pub fn watch(user: &User) {
    match add_watchee(&user.id) {
        Ok(_) => {
            dm_facade(user, "Now I'm watching you!");
        }
        Err(_) => {
            dm_facade(user, "I'm already watching you.");
        }
    }
}

// change status of msg's author to unwatching and say so
pub fn unwatch(user: &User) {
    match remove_watchee(&user.id) {
        Ok(_) => {
            dm_facade(user, "I'm no longer watching you.");
        }
        Err(_) => {
            dm_facade(user, "I'm not watching you.");
        }
    }
}

pub fn status(msg: &Message) {
    talk_facade(&msg.channel_id, match has_watchee(&msg.author.id) {
        Some(_) => "I'm watching you.",
        None => "I'm not watching you."
    });
}

// capture a watching player's status change and dm
pub fn stat_update(game: Option<&Game>, player: &Watchee) {
    let user = &player.to_user().expect("failed to get user data");
    match (game, player.game_as_option()) {
        (Some(new_game), Some(old_game)) => {
            dm_facade(&user, &*format!("You started {} and thus quit {}, which had played for {} minutes.", new_game.name, old_game.name, minutes(player.timestamp_as_instant())));
            update_game(player, Some(new_game.clone()));
        },
        (Some(new_game), None) => {
            dm_facade(&user, &*format!("You started {}.", new_game.name));
            update_game(player, Some(new_game.clone()));
        },
        (None, Some(old_game)) => {
            dm_facade(&user, &*format!("You have played {} for {} minutes.", old_game.name, minutes(player.timestamp_as_instant())));
            update_game(player, None);
        },
        (None, None) => ()
    }
}

// TODO remove when release 
// list up watchee info
pub fn list() {
    for (i, watchee) in get_lock().iter().enumerate() {
        println!("watchees No.{}| id:{:?}, status:{:?}, game:{:?}, timestamp:{:?}", 
                i, watchee.id_as_u64(), watchee.stat_as_enum(), watchee.game_as_string(), watchee.timestamp_as_instant());
    }
}
