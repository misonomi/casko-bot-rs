use std::fmt;

use rand::{ Rng, seq::SliceRandom };

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

#[derive(Debug, Clone, Copy)]
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
    fn gen_trinity() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 3) {
            0 => Card::Attack, 
            1 => Card::Break, 
            2 => Card::Guard, 
            _ => Card::Error,
        }
    }
    fn gen_skill() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 2) {
            0 => Card::Kokutendo, 
            1 => Card::Kuretsu, 
            _ => Card::Error,
        }
    }
    fn gen_all() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 5) {
            0 => Card::Attack, 
            1 => Card::Break, 
            2 => Card::Guard, 
            3 => Card::Kokutendo, 
            4 => Card::Kuretsu, 
            _ => Card::Error,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Card::Attack => "⚔️",
            Card::Break => "💥",
            Card::Guard => "🛡️",
            Card::Kokutendo => "a",
            Card::Kuretsu => "b",
            Card::Wild => ".",
            Card::Error => "❓",
        })
    }
}

impl PartialEq for Card {
    fn eq(&self, tgt: &Self) -> bool {
        match (self, tgt) {
            (Card::Attack, Card::Attack) => true,
            (Card::Break, Card::Break) => true,
            (Card::Guard, Card::Guard) => true,
            (Card::Kokutendo, Card::Kokutendo) => true,
            (Card::Kuretsu, Card::Kuretsu) => true,
            (Card::Wild, Card::Wild) => true,
            (Card::Error, Card::Error) => true,
            _ => false,
        }
    }
}

impl Default for Card {
    fn default() -> Self { Card::Error }
}

const HANDLEN : usize = 6;

#[derive(Debug, Clone, Default)]
pub struct Hand {
    card: Card,
    visible: bool,
}

impl Hand {
    fn generate_easy() -> [Self; HANDLEN] {
        let card_list: [Card; HANDLEN] = [Card::Attack, Card::Break, Card::Guard, Card::Kokutendo, Card::gen_trinity(), Card::gen_trinity()];
        let visible_list = [true, true, true, true, false, false];

        Hand::generate_core(card_list, visible_list)
    }
    fn generate_normal() -> [Self; HANDLEN] {
        let mut rng = rand::thread_rng();
        let card_list: [Card; HANDLEN] = [Card::Break, Card::gen_trinity(), Card::gen_trinity(), Card::gen_trinity(), Card::gen_trinity(), Card::gen_skill()];
        let visible_list = [true, true, true, rng.gen(), false, false];

        Hand::generate_core(card_list, visible_list)
    }
    fn generate_hard() -> [Self; HANDLEN] {
        let mut rng = rand::thread_rng();
        let card_list: [Card; HANDLEN] = [Card::gen_trinity(), Card::gen_trinity(), Card::gen_trinity(), Card::gen_trinity(), Card::gen_all(), Card::gen_skill()];
        let visible_list = [true, rng.gen_bool(0.7), rng.gen(), rng.gen(), false, false];

        Hand::generate_core(card_list, visible_list)
    }
    fn generate_core(mut card: [Card; HANDLEN], mut vis: [bool; HANDLEN]) -> [Self; HANDLEN] {
        let mut rng = rand::thread_rng();
        card.shuffle(&mut rng);
        vis.shuffle(&mut rng);

        let mut hand_list: [Self; HANDLEN] = Default::default();
        for i in 0..HANDLEN {
            hand_list[i] = Hand{ card: card[i], visible: vis[i] };
        }
        hand_list
    }

    fn interpret(msg: &String) -> [Card; HANDLEN] {
        let mut hands = [Card::Error; HANDLEN];
        let mut ptr: isize = 0;
        for c in msg.chars() {
            match c {
                'A' | 'a' => hands[ptr as usize] = Card::Attack,
                'B' | 'b' => hands[ptr as usize] = Card::Break,
                'G' | 'g' => hands[ptr as usize] = Card::Guard,
                'S' | 's' => hands[ptr as usize] = Card::Kokutendo,
                '.' => hands[ptr as usize] = Card::Wild,
                ':' | ' ' | ',' | '>' | '|' => ptr -= 1,
                _ => hands[ptr as usize] = Card::Error,
            }
            ptr += 1;
            if ptr as usize >= HANDLEN {
                break;
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
    next_hand: [Hand; HANDLEN],
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

    fn generate(&mut self) -> &[Hand; HANDLEN] {
        match self.diffic {
            Difficulty::Easy => self.next_hand = Hand::generate_easy(),
            Difficulty::Normal => self.next_hand = Hand::generate_normal(),
            Difficulty::Hard => self.next_hand = Hand::generate_hard(),
        }
        &self.next_hand
    }

    fn judge(&mut self, i: usize, pcard: &mut Card) -> TurnResult {
        let ccard = &self.next_hand[i];
        match (&pcard, &ccard.card, &ccard.visible) {
            (Card::Attack, Card::Attack, _) => self.draw(100),
            (Card::Attack, Card::Break, _) => self.lose(100),
            (Card::Attack, Card::Guard, _) => self.win(100),
            (Card::Attack, Card::Kokutendo, _) => self.kokutendo_c(100),
            (Card::Attack, Card::Kuretsu, _) => self.win(100),

            (Card::Break, Card::Attack, _) => self.win(100),
            (Card::Break, Card::Break, _) => self.draw(100),
            (Card::Break, Card::Guard, _) => self.lose(100),
            (Card::Break, Card::Kokutendo, _) => self.kokutendo_c(100),
            (Card::Break, Card::Kuretsu, _) => self.win(100),

            (Card::Guard, Card::Attack, _) => self.lose(100),
            (Card::Guard, Card::Break, _) => self.win(100),
            (Card::Guard, Card::Guard, _) => self.draw(0),
            (Card::Guard, Card::Kokutendo, _) => self.draw(0),
            (Card::Guard, Card::Kuretsu, _) => self.win(100),

            (Card::Kokutendo, Card::Attack, _) => self.kokutendo_p(100),
            (Card::Kokutendo, Card::Break, _) => self.kokutendo_p(100),
            (Card::Kokutendo, Card::Guard, _) => self.draw(0),
            (Card::Kokutendo, Card::Kokutendo, _) => self.draw(0),
            (Card::Kokutendo, Card::Kuretsu, _) => self.kokutendo_p(100),

            (Card::Wild, Card::Attack, true) => self.lose(100),
            (Card::Wild, Card::Break, true) => self.lose(100),
            (Card::Wild, Card::Guard, true) => self.lose(100),
            (Card::Wild, Card::Kokutendo, true) => self.draw(0),
            (Card::Wild, Card::Kuretsu, true) => self.kokutendo_p(100),
            (Card::Wild, _, false) => self.win(100),

            (Card::Error, Card::Attack, _) => self.win(100),
            (Card::Error, Card::Break, _) => self.win(100),
            (Card::Error, Card::Guard, _) => self.draw(0),
            (Card::Error, Card::Kokutendo, _) => self.draw(0),
            (Card::Error, Card::Kuretsu, _) => self.win(100),

            _ => { println!("unexpected pattern :{}, {}", pcard, ccard.card); TurnResult::None },
        }
    }
    
    fn win(&mut self, damage_base: u16) -> TurnResult {
        self.player_health -= damage_base;
        TurnResult::Win
    }
    fn lose(&mut self, damage_base: u16) -> TurnResult {
        self.casko_health -= damage_base;
        TurnResult::Lose
    }
    fn draw(&mut self, damage_base: u16) -> TurnResult {
        self.player_health -= damage_base;
        self.casko_health -= damage_base;
        TurnResult::Draw
    }
    fn kokutendo_p(&mut self, damage_base: u16) -> TurnResult {
        self.player_health -= damage_base / 10;
        TurnResult::Guard
    }
    fn kokutendo_c(&mut self, damage_base: u16) -> TurnResult {
        self.casko_health -= damage_base / 10;
        
        TurnResult::Guard
    }
}

enum TurnResult {
    Win,
    Lose,
    Draw,
    Guard,
    None,
}

impl fmt::Display for TurnResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            TurnResult::Win => "WIN",
            TurnResult::Lose => "LOSE",
            TurnResult::Draw => "DRAW",
            TurnResult::Guard => "KOKUTENDO",
            TurnResult::None => "?",
        })
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
    talk_facade(&msg.channel_id, &*deal(&msg.author.id));
    true
}

pub fn battle(msg: &Message) -> bool {
    if let Some(TalkSequence::InCombat(mut combat_stat)) = meltomos::get_seq(&msg.author.id) {
        let mut player_card = Hand::interpret(&msg.content);
        for i in 0..HANDLEN - 1 {
            let (pcard, ccard) = (&mut player_card[i as usize], combat_stat.next_hand[i as usize].card);
            let (ph_before, ch_before) = (combat_stat.player_health, combat_stat.casko_health);
            let result = combat_stat.judge(i, pcard);
            let (ph_after, ch_after) = (combat_stat.player_health, combat_stat.casko_health);

            talk_facade(&msg.channel_id, &*format!("turn {}", i + 1));
            talk_facade(&msg.channel_id, &*format!("[you : {}] vs [{} : me]", pcard, ccard));
            talk_facade(&msg.channel_id, &*format!(" HP: {} -> {} -+-[{}]-+- HP: {} -> {}", ph_before, ph_after, result, ch_before, ch_after));

            if ph_after <= 0 && ch_after <= 0 {
                draw(msg, &combat_stat.diffic);
            } else if ph_after <= 0 {
                win(msg, &combat_stat.diffic);
                return true;
            } else if ch_after <= 0 {
                lose(msg, &combat_stat.diffic);
                return true;
            }
        }
        meltomos::update_seq(&msg.author.id, TalkSequence::InCombat(combat_stat));
        talk_facade(&msg.channel_id, &*deal(&msg.author.id));
        true
    } else {
        false
    }
}

fn deal(id: &UserId) -> String {
    if let Some(TalkSequence::InCombat(mut combat_stat)) = meltomos::get_seq(id) {
        combat_stat.generate();
        let mut parsed_hand = String::from("|");
        for hand in combat_stat.next_hand.iter() {
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
        meltomos::update_seq(id, TalkSequence::InCombat(combat_stat));
        parsed_hand
    } else {
        String::from("")
    }
}

fn win(msg: &Message, stat: &Difficulty) {
    meltomos::update_seq(&msg.author.id, TalkSequence::None);
    match stat {
        Difficulty::Easy => {
            talk_facade(&msg.channel_id, "yay!");
        },
        Difficulty::Normal => {
            talk_facade(&msg.channel_id, "yay!");

        },
        Difficulty::Hard => {
            talk_facade(&msg.channel_id, "yay!");

        },
    }
}

fn lose(msg: &Message, stat: &Difficulty) {
    meltomos::update_seq(&msg.author.id, TalkSequence::None);
    match stat {
        Difficulty::Easy => {
            talk_facade(&msg.channel_id, "congrats!");
        },
        Difficulty::Normal => {
            talk_facade(&msg.channel_id, "congrats!");

        },
        Difficulty::Hard => {
            talk_facade(&msg.channel_id, "congrats!");

        },
    }
}

fn draw(msg: &Message, _stat: &Difficulty) {
    meltomos::update_seq(&msg.author.id, TalkSequence::None);
    talk_facade(&msg.channel_id, "hikiwake");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpret() {
        test_interpret_core(">>A,B G|S:.X", [Card::Attack, Card::Break, Card::Guard, Card::Kokutendo, Card::Wild, Card::Error]);
        test_interpret_core("a,bg ,s.   x", [Card::Attack, Card::Break, Card::Guard, Card::Kokutendo, Card::Wild, Card::Error]);
        test_interpret_core("hello,world!", [Card::Error, Card::Error, Card::Error, Card::Error, Card::Error, Card::Error]);
        test_interpret_core(">AAA>", [Card::Attack, Card::Attack, Card::Attack, Card::Error, Card::Error, Card::Error]);
    }

    fn test_interpret_core(text: &str, hands: [Card; HANDLEN]) {
        let result = Hand::interpret(&String::from(text));
        for i in 0..HANDLEN - 1 {
            assert_eq!(result[i], hands[i]);
        }
    }
}
