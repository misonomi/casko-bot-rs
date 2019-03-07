use serenity::model::{ channel::Message, gateway::Presence, user::User };

use crate::watchees::*;

// change status of msg's author to watching and say so
pub fn watch(user: &User) {
    add_watchee(user.id.as_u64());
}

// change status of msg's author to unwatching and say so
pub fn unwatch(user: &User) {
    remove_watchee(user.id.as_u64());
}

// capture a watching player's status change and dm
pub fn stat_update(msg: &Presence) {

}

// for debugging: list up watchee id 
pub fn list() {
    
}

