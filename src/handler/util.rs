use std::time::Instant;
use lazy_static::lazy_static;
use regex::Regex;

use serenity::{
    model::{ id::ChannelId, user::User, channel::{ Message, ReactionType } },
    prelude::Context,
};

lazy_static! {
    static ref PREFIX: Regex = Regex::new(r"^\s*\^\^~\s*(.*)").expect("wrong pattern!!!!!!!!!");
    pub static ref HAND_PATTERN: Regex = Regex::new(r"^>>.*").expect("wrong pattern!!!!!!!!!");
}

pub fn dm_facade(c: &Context, user: &User, mes: &str) {
    if let Err(cause) = user.dm(c, |m| m.content(mes)) {
        println!("Error when direct messaging user: {:?}", cause);
    } else {
        println!("dmed: {}", mes);
    }
}

pub fn talk_facade(c: &Context, channel: &ChannelId, mes: &str) {
    if let Err(cause) = channel.say(c, mes) {
        println!("Error when talking: {:?}", cause);
    } else {
        println!("talked: {}", mes);
    }
}

pub fn react_facade(c: &Context, mes: &Message, unicode: &str) {
    if let Err(cause) = mes.react(c, ReactionType::from(unicode)) {
        println!("Error when reacting: {:?}", cause);
    }
}

pub fn minutes(from: Instant) -> u64 {
    from.elapsed().as_secs() / 60
}

pub fn remove_prefix(mes: &str) -> Option<&str> {
    if let Some(text) = PREFIX.captures(mes) {
        return text.get(1).map(|m| m.as_str());
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minutes() {
    }

    #[test]
    fn test_has_prefix() {
        test_has_prefix_core("not accepted", None);
        test_has_prefix_core("", None);
        test_has_prefix_core("^^~", Some(""));
        test_has_prefix_core("^^~ok", Some("ok"));
        test_has_prefix_core("^^~  ok", Some("ok"));
        test_has_prefix_core("  ^^~  ok", Some("ok"));
        test_has_prefix_core("^^~^^~", Some("^^~"));
    }

    fn test_has_prefix_core(str_in: &str, str_out: Option<&str>) {
        assert_eq!(remove_prefix(str_in), str_out);
    }
}
