use serenity::model::{ channel::Message, user::User };

use crate::watchees::*;

pub fn help(msg: &Message) {
    if let Err(cause) = msg.author.dm(|m| m.content("*help*")) {
        println!("Error when direct messaging user: {:?}", cause);
    }
    if let Err(cause) = msg.channel_id.say("sent help to your direct message") {
        println!("Error when direct messaging user: {:?}", cause);
    }
}

pub fn status(msg: &Message) {
    if let Err(cause) = msg.channel_id.say("sent help to your direct message") {
        println!("Error when direct messaging user: {:?}", cause);
    }
}

pub fn welcome(user: &User) {
    if has_watchee(user.id.as_u64()).is_some() {
        if let Err(cause) = user.dm(|m| m.content("*help*")) {
            println!("Error when direct messaging user: {:?}", cause);
        }
    }
}
