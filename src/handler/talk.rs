use serenity::model::{ channel::Message };

use super::util::{ dm_facade, react_facade };

pub fn help(msg: &Message) {
    react_facade(msg, "✅");
    dm_facade(&msg.author, "*help*");
}

pub fn command_battle(msg: &Message) {
    react_facade(msg, "🔥");
    dm_facade(&msg.author, "I accept your challenge. choose difficulty.");
    dm_facade(&msg.author, "say e(easy) / n(normal) / h(hard)");
}

// TODO delete when its not needed
pub fn whois(msg: &Message) {
    react_facade(msg, "✅");
    dm_facade(&msg.author, &*format!("your ID is {}", msg.author.id));
}

pub fn dunno(msg: &Message) {
    react_facade(msg, "🤔");
    dm_facade(&msg.author, "unknown command. say 'help' to get help dm");
}
