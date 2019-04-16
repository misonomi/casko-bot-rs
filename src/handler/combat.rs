use serenity::model::{ channel::Message, id::UserId };

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
    Error,
}

impl Card {
    fn to_emoji(&self) -> &str {
        match self {
            Attack => "⚔️",
            Break => "💥",
            Guard => "🛡️",
            Kokutendo => "",
            Kuretsu => "",
            Wild => "",
            Error => "❓",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    card: Card,
    visible: bool,
}

impl Hand {
    fn generate_easy() -> Vec<Self> {
        let mut hands = Vec::new();

        hands
    }
    fn generate_normal() -> Vec<Self> {
        let mut hands = Vec::new();

        hands
    }
    fn generate_hard() -> Vec<Self> {
        let mut hands = Vec::new();

        hands
    }
    fn interpret(msg: &String) -> Vec<Card> {
        let mut hands = Vec::new();
        for c in msg.chars().rev() {
            match c {
                'A' | 'a' => hands.push(Card::Attack),
                'B' | 'b' => hands.push(Card::Break),
                'G' | 'g' => hands.push(Card::Guard),
                'S' | 's' => hands.push(Card::Kokutendo),
                '.' => hands.push(Card::Wild),
                '\t' | ' ' | ',' => (),
                _ => hands.push(Card::Error),
            }
        }
        hands
    }
}

#[derive(Debug, Clone)]
pub struct CombatStatus {
    diffic: Difficulty,
    player_health: u16,
    casko_health: u16,
    next_hand: Vec<Hand>,
}

impl CombatStatus {
    fn init(diffic: Difficulty) -> Self {
        let (ph, ch, nh) = match diffic {
            Difficulty::Easy => (400, 200, Hand::generate_easy()),
            Difficulty::Normal => (400, 200, Hand::generate_normal()),
            Difficulty::Hard => (400, 200, Hand::generate_hard()),
        };
        CombatStatus{ diffic: diffic, player_health: ph, casko_health: ch, next_hand: nh }
    }

    fn generate(&mut self) -> &Vec<Hand> {
        match self.diffic {
            Difficulty::Easy => self.next_hand = Hand::generate_easy(),
            Difficulty::Normal => self.next_hand = Hand::generate_normal(),
            Difficulty::Hard => self.next_hand = Hand::generate_hard(),
        }
        &self.next_hand
    }

    fn judge(&mut self, i: u64, pcard: &Card) -> (Option<bool>, u64, Card, Card) {

        (None, 0, Card::Attack, Card::Attack)
    }
}

pub fn choose(msg: &Message, diffic: Difficulty) -> bool {
    if !meltomos::conjecture_seq(&msg.author.id, TalkSequence::ChooseDiffic) { return false; }
    match diffic {
        Difficulty::Easy => {
            react_facade(msg, "✅");
            talk_facade(&msg.channel_id, "");
        },
        Difficulty::Normal => {
            react_facade(msg, "");
            talk_facade(&msg.channel_id, "");
        },
        Difficulty::Hard => {
            react_facade(msg, "");
            talk_facade(&msg.channel_id, "");
        },
    }
    meltomos::update_seq(&msg.author.id, TalkSequence::InCombat(CombatStatus::init(diffic)));
    talk_facade(&msg.channel_id, &*hand_info(&msg.author.id));
    true
}

pub fn battle(msg: &Message) -> bool {
    if let Some(TalkSequence::InCombat(mut battle_stat)) = meltomos::get_seq(&msg.author.id) {
        let player_card = Hand::interpret(&msg.content);
        for i in 0..5 {
            let (winner, damage, pcard, ccard) = battle_stat.judge(i, &player_card[i as usize]);
            talk_facade(&msg.channel_id, &*format!("turn {}", i + 1));
            talk_facade(&msg.channel_id, &*format!("turn {}", i + 1));
        }
        true
    } else {
        false
    }
}

fn hand_info(id: &UserId) -> String {
    if let Some(TalkSequence::InCombat(mut battle_stat)) = meltomos::get_seq(id) {
        battle_stat.generate();
        let mut parsed_hand = String::from("|");
        for hand in battle_stat.next_hand {
            if hand.visible {
                match hand.card {
                    Card::Attack => parsed_hand.push_str("ATK"),
                    Card::Break => parsed_hand.push_str("BRK"),
                    Card::Guard => parsed_hand.push_str("GRD"),
                    Card::Kokutendo => parsed_hand.push_str("SKILL"),
                    Card::Kuretsu => parsed_hand.push_str("SKILL"),
                    _ => (),
                }
            } else {
                parsed_hand.push_str("?");
            }
            parsed_hand.push_str("|");
        }
        parsed_hand
    } else {
        String::from("")
    }
}
