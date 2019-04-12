use std::time::Instant;

use serenity::model::{ id::UserId, gateway::Game, user::User };

use super::stat;

pub struct Meltomo {
    pub id: UserId,
    pub stat: stat::BondType,
    pub seq: stat::TalkSequence,
    pub game: Option<Game>,
    pub last_update: Instant
}

impl Meltomo {
    pub fn new(id: UserId, stat: stat::BondType) -> Meltomo {
        Meltomo{ 
            id: id, 
            stat: stat, 
            seq: stat::TalkSequence::None, 
            game: None, 
            last_update: Instant::now(),
        }
    }

    pub fn to_user(&self) -> serenity::Result<User> {
        self.id.to_user()
    }

    pub fn has_id(&self, id: &UserId) -> bool {
        self.id.as_u64() == id.as_u64()
    }

    pub fn change_stat(&mut self, stat: stat::BondType) -> Result<(), ()> {
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
