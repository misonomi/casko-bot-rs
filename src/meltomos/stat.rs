use std::u8::MAX;

use crate::handler::combat::Difficulty;

#[derive(Debug, Clone)]
pub enum BondType {
    Normal,
    Watching,
    Admin,
    Unknown
}

impl BondType {
    pub fn into_borrow(&self) -> &u8 {
        match self {
            BondType::Normal => &1,
            BondType::Watching => &2,
            BondType::Admin => &0,
            BondType::Unknown => &MAX
        }
    }
}

impl From<u8> for BondType {
    fn from(num: u8) -> Self {
        match num {
            1 => BondType::Normal,
            2 => BondType::Watching,
            0 => BondType::Admin,
            _ => BondType::Unknown
        }
    }
}

impl Into<u8> for BondType {
    fn into(self) -> u8 {
        match self {
            BondType::Normal => 1,
            BondType::Watching => 2,
            BondType::Admin => 0,
            BondType::Unknown => MAX
        }
    }
}

impl PartialEq for BondType {
    fn eq(&self, target: &BondType) -> bool {
        self.into_borrow() == target.into_borrow()
    }
}

#[derive(Debug, Clone)]
pub enum TalkSequence {
    None,
    ChooseDiffic,
    InCombat(Difficulty, u16, u16),
}

impl TalkSequence {
    fn into_borrow(&self) -> &u8 {
        match self {
            TalkSequence::None => &0,
            TalkSequence::ChooseDiffic => &1,
            TalkSequence::InCombat(_, _, _) => &2,
        }
    }
}

impl PartialEq for TalkSequence {
    fn eq(&self, target: &TalkSequence) -> bool {
        self.into_borrow() == target.into_borrow()
    }
}
