use std::fs::File;
use std::io::{ BufReader, BufRead, Error };
use std::sync::Mutex;
use lazy_static::lazy_static;

use serenity::model::{ id::UserId, gateway::Game };

pub struct Watchee {
    id: UserId,
    game: Option<Game>
}

lazy_static! {
    static ref WATCHEES: Mutex<Vec<Watchee>> = {
        let mut watchlist = Vec::new();
        
        let watchee_reader = BufReader::new(File::open("watchees.dat").expect("no file: 'watchees.dat'"));
        for raw_watchee in watchee_reader.lines() {
            watchlist.push(Watchee{ id: interpret_line(raw_watchee), game: None });
        }
        Mutex::new(watchlist)
    };
}

fn interpret_line(_lineresult: Result<String, Error>) -> UserId {
    UserId::from(1)
}

pub fn add_watchee(id: &UserId) -> Result<usize, usize> {
    if has_watchee(id).is_none() {
        let mut watchees_guarded = WATCHEES.lock().unwrap();
        watchees_guarded.push(Watchee{ id: *id, game: None });
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
