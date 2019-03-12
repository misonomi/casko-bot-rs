use serenity::model::{ gateway::Presence, user::User };

use crate::watchees::*;
use super::util::dm_facade;

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

// capture a watching player's status change and dm
pub fn stat_update(presence: &Presence) {
    
}

// TODO remove when release 
// list up watchee id 
pub fn list() {
    
}

