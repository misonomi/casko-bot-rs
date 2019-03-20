use serenity::model::{ channel::Message };

use super::util::{ dm_facade, talk_facade };

pub fn help(msg: &Message) {
    dm_facade(&msg.author, "*help*");
    talk_facade(&msg.channel_id, "sent help to you by direct message");
}

// TODO delete when its not needed
pub fn whois(msg: &Message) {
    dm_facade(&msg.author, &*format!("your ID is {}", msg.author.id));
}
