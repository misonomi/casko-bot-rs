use std::fs::File;
use std::io::{ BufReader, BufRead, Error };
use std::sync::{ Mutex, MutexGuard };
use std::time::Instant;
use lazy_static::lazy_static;

use serenity::model::{ id::UserId, gateway::Game };

mod stat;

pub struct Watchee {
    id: UserId,
    stat: stat::BondType,
    game: Option<Game>,
    last_update: Instant
}

impl Watchee {
    pub fn id_as_u64(&self) -> &u64 {
        &self.id.as_u64()
    }
    pub fn stat_as_enum(&self) -> &stat::BondType {
        &self.stat
    }
    pub fn game_as_string(&self) -> Option<&String> {
        match &self.game {
            Some(game) => Some(&game.name),
            None => None
        }
    }
    pub fn timestamp_as_instant(&self) -> &Instant {
        &self.last_update
    }
}

lazy_static! {
    static ref WATCHELIST: Mutex<Vec<Watchee>> = {
        let mut watchlist = Vec::new();
        
        let watchee_reader = BufReader::new(File::open("watchees.dat").expect("no file: 'watchees.dat'"));
        for raw_watchee in watchee_reader.lines() {
            if let Some(watchee) = interpret_line(&raw_watchee) {
                watchlist.push(watchee);
            }
        }
        Mutex::new(watchlist)
    };
}

fn interpret_line(line: &Result<String, Error>) -> Option<Watchee> {
    // TODO lines().expect() and not match?
    // do when I wont need cause anymore
    match line {
        Ok(line) =>{
            let raw_props: Vec<&str> = line.split(":").collect();
            if raw_props.len() != 2 {
                return None;
            }
            let u64id: u64 = raw_props[0].parse().expect("failed to parse userid");
            let u8stat: u8 = raw_props[1].parse().expect("failed to parse stat");
            Some(Watchee{ id: UserId::from(u64id), stat: stat::bond_from(u8stat), game: None, last_update: Instant::now() })
        },
        Err(cause) => {
            println!("Error when reading watchees: {:?}", cause);
            None
        }
    }
}

pub fn add_watchee(id: &UserId) -> Result<usize, usize> {
    if has_watchee(id).is_none() {
        let mut watchees_guarded = WATCHELIST.lock().unwrap();
        watchees_guarded.push(Watchee{ id: *id, stat: stat::BondType::normal, game: None, last_update: Instant::now() });
        Ok(watchees_guarded.capacity())
    } else {
        Err(0)
    }
}

pub fn remove_watchee(id: &UserId) -> Result<usize, usize> {
    if let Some(pos) = has_watchee(id) {
        WATCHELIST.lock().unwrap().remove(pos);
        Ok(pos)
    } else {
        Err(0)
    }
}

pub fn get_watchlist() -> MutexGuard<'static, Vec<Watchee>> {
    WATCHELIST.lock().unwrap()
}

pub fn has_watchee(id: &UserId) -> Option<usize> {
    WATCHELIST.lock().unwrap().iter().position(|x| *x.id.as_u64() == *id.as_u64() )
}

pub fn game_changed(id: &UserId, game: &Option<Game>) -> Result<bool, Error> {
    Ok(true)
}
