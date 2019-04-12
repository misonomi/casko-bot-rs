use std::time::Instant;

use serenity::model::{ id::UserId, gateway::Game };

use super::stat::{ BondType, TalkSequence };

pub struct Meltomo {
    pub id: UserId,
    pub stat: BondType,
    pub seq: TalkSequence,
    pub game: Option<Game>,
    pub last_update: Instant
}

impl Meltomo {
    pub fn new(id: UserId, stat: BondType) -> Meltomo {
        Meltomo{ 
            id: id, 
            stat: stat, 
            seq: TalkSequence::None, 
            game: None, 
            last_update: Instant::now(),
        }
    }

    pub fn has_id(&self, id: &UserId) -> bool {
        self.id.as_u64() == id.as_u64()
    }

    pub fn change_stat(&mut self, stat: BondType) -> Result<(), ()> {
        if self.stat == stat {
            Err(())
        } else {
            self.stat = stat;
            Ok(())
        }
    }

    pub fn update_game(&mut self, game: Option<Game>) {
        self.game = game;
        self.last_update = Instant::now()
    }

    pub fn game_changed(&self, new_game: Option<&Game>) -> bool {
        match (self.game.as_ref(), new_game) {
            (Some(old), Some(new)) => old.name != new.name,
            (None, None) => false,
            _ => true,
        }
    }
}

impl PartialEq for Meltomo {
    fn eq(&self, target: &Meltomo) -> bool {
        self.id.as_u64() == target.id.as_u64()
    }
}
