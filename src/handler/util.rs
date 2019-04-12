use std::time::Instant;
use lazy_static::lazy_static;
use regex::Regex;

use serenity::model::{ id::ChannelId, user::User, channel::{ Message, ReactionType } };

pub fn dm_facade(user: &User, mes: &str) {
    if let Err(cause) = user.dm(|m| m.content(mes)) {
        println!("Error when direct messaging user: {:?}", cause);
    }
}

pub fn talk_facade(channel: &ChannelId, mes: &str) {
    if let Err(cause) = channel.say(mes) {
        println!("Error when talking: {:?}", cause);
    }
}

pub fn react_facade(mes: &Message, unicode: &str) {
    if let Err(cause) = mes.react(ReactionType::from(unicode)) {
        println!("Error when reacting: {:?}", cause);
    }
}

pub fn minutes(from: Instant) -> u64 {
    from.elapsed().as_secs() / 60
}

pub fn remove_prefix(mes: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*\^\^~\s*(.*)").expect("wrong prefix!!!!!!!!!");
    }
    if let Some(text) = RE.captures(mes) {
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
        thp_core("not accepted", None);
        thp_core("", None);
        thp_core("^^~", Some(""));
        thp_core("^^~ok", Some("ok"));
        thp_core("^^~  ok", Some("ok"));
        thp_core("  ^^~  ok", Some("ok"));
        thp_core("^^~^^~", Some("^^~"));
    }

    fn thp_core(str_in: &str, str_out: Option<&str>) {
        assert_eq!(remove_prefix(str_in), str_out);
    }
}
