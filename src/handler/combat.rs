use serenity::model::{ channel::Message };

use super::util::{ talk_facade, react_facade };
use crate::meltomos;
use crate::meltomos::stat::TalkSequence;

#[derive(Debug, Clone)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Clone)]
enum Card {
    Attack,
    Break,
    Guard,
    Kokutendo,
    Kuretsu,
    Wild,
}

#[derive(Debug, Clone)]
struct Hand {
    card: Card,
    visible: bool,
}

impl Hand {
    fn generate_easy(i: u64) -> Self {

    }
    fn generate_normal(i: u64) -> Self {

    }
    fn generate_hard(i: u64) -> Self {

    }
}

pub fn choose(msg: &Message, diffic: Difficulty) -> bool {
    if !meltomos::conjecture_seq(&msg.author.id, TalkSequence::ChooseDiffic) { return false; }
    match diffic {
        Difficulty::Easy => {
            react_facade(msg, "✅");
            talk_facade(&msg.channel_id, "");
            meltomos::update_seq(&msg.author.id, TalkSequence::InCombat(Difficulty::Easy, 1, 1));
        },
        Difficulty::Normal => {
            react_facade(msg, "✅");
            talk_facade(&msg.channel_id, "");
            meltomos::update_seq(&msg.author.id, TalkSequence::InCombat(Difficulty::Easy, 1, 1));
        },
        Difficulty::Hard => {
            react_facade(msg, "✅");
            talk_facade(&msg.channel_id, "");
            meltomos::update_seq(&msg.author.id, TalkSequence::InCombat(Difficulty::Easy, 1, 1));
        },
    }
    true
}

pub fn generate_hands(diffic: Difficulty) -> Vec<Hand> {
    let mut hands = Vec::new();
    for i in 0..5 {
        hands.push(match diffic {
            Difficulty::Easy => Hand::generate_easy(i),
            Difficulty::Normal => Hand::generate_easy(i),
            Difficulty::Hard => Hand::generate_easy(i),
        })
    }
    hands
}

pub fn battle(msg: &Message) -> bool {
    if !meltomos::conjecture_seq(&msg.author.id, TalkSequence::InCombat(_, _,_)) { return false; }
    true;
}
