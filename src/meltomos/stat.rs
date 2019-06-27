use std::u8::MAX;

use crate::handler::combat::CombatStatus;

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
    InCombat(CombatStatus),
    Vote,
    FreeTalk,
}

impl PartialEq for TalkSequence {
    fn eq(&self, target: &TalkSequence) -> bool {
        match (self, target) {
            (TalkSequence::None, TalkSequence::None) => true,
            (TalkSequence::ChooseDiffic, TalkSequence::ChooseDiffic) => true,
            (TalkSequence::InCombat(_), TalkSequence::InCombat(_)) => true,
            (TalkSequence::FreeTalk, TalkSequence::FreeTalk) => true,
            _ => false,
        }
    }
}
