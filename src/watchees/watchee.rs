use std::time::Instant;

use serenity::model::{ id::UserId, gateway::Game, user::User };

use super::stat;

pub struct Watchee {
    id: UserId,
    stat: stat::BondType,
    seq: stat::TalkSequence,
    game: Option<Game>,
    last_update: Instant
}

impl Watchee {
    pub fn new(id: UserId, stat: stat::BondType) -> Watchee {
        Watchee{ 
            id: id, 
            stat: stat, 
            seq: stat::TalkSequence::NONE, 
            game: None, 
            last_update: Instant::now(),
        }
    }

    pub fn incarnate(&self) -> Watchee {
        Watchee{ 
            id: self.id.clone(), 
            stat: self.stat.clone(), 
            seq: self.seq.clone(),  
            game: self.game.clone(), 
            last_update: self.last_update.clone(), 
        }
    }

    pub fn to_user(&self) -> serenity::Result<User> {
        self.id.to_user()
    } 
    pub fn id_as_u64(&self) -> &u64 {
        self.id.as_u64()
    }
    pub fn stat_as_enum(&self) -> &stat::BondType {
        &self.stat
    }
    pub fn stat_as_u8(&self) -> &u8 {
        stat::bond_to(&self.stat)
    }
    pub fn game_as_string(&self) -> Option<&String> {
        match &self.game {
            Some(game) => Some(&game.name),
            None => None
        }
    }
    pub fn game_as_option(&self) -> Option<&Game> {
        self.game.as_ref()
    }
    pub fn timestamp_as_instant(&self) -> &Instant {
        &self.last_update
    }

    pub fn update_game(&mut self, game: Option<Game>) {
        self.game = game;
        self.last_update = Instant::now()
    }

    pub fn game_changed(&self, new_game: &Option<Game>) -> bool {
        if self.game.is_some() && new_game.is_some() {
            &self.game.as_ref().unwrap().name != &new_game.as_ref().unwrap().name
        } else if self.game.is_none() && new_game.is_none() {
            false
        } else {
            true
        }
    }
}

impl PartialEq for Watchee {
    fn eq(&self, target: &Watchee) -> bool {
        self.id_as_u64() == target.id_as_u64()
    }
}
