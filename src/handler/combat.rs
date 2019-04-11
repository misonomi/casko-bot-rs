use serenity::model::{ channel::Message };

use super::util::{ dm_facade, react_facade };

pub enum Difficulty {
    EASY,
    NORMAL,
    HARD,
}

pub fn choose(msg: &Message, diffic: Difficulty) -> bool {
    /*
    if msg.author.
    react_facade(msg, "");
    dm_facade(&msg.author, "*help*");
    */
    true
}
