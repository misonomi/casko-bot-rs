use std::fs::File;
use std::io::{ BufReader, BufRead, Error };
use std::sync::Mutex;
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

lazy_static! {
    static ref WATCHEES: Mutex<Vec<Watchee>> = {
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
        let mut watchees_guarded = WATCHEES.lock().unwrap();
        watchees_guarded.push(Watchee{ id: *id, stat: stat::BondType::normal, game: None, last_update: Instant::now() });
        Ok(watchees_guarded.capacity())
    } else {
        Err(0)
    }
}

pub fn remove_watchee(id: &UserId) -> Result<usize, usize> {
    let pos = has_watchee(id);
    if pos.is_some() {
        WATCHEES.lock().unwrap().remove(pos.unwrap());
        Ok(pos.unwrap())
    } else {
        Err(0)
    }
}

pub fn has_watchee(id: &UserId) -> Option<usize> {
    WATCHEES.lock().unwrap().iter().position(|x| x.id.as_u64() == id.as_u64() )
}

pub fn game_changed(id: &UserId, game: &Option<Game>) -> Result<bool, Error> {
    Ok(true)
}
