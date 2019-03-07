use std::fs::File;
use std::io::{ BufReader, BufRead, Error };
use std::sync::Mutex;
use lazy_static::lazy_static;

pub struct Watchee {
    id: u64
}

lazy_static! {
    static ref WATCHEES: Mutex<Vec<Watchee>> = {
        let mut watchlist = Vec::new();
        
        let watchee_reader = BufReader::new(File::open("watchees.dat").expect("no file: 'watchees.dat'"));
        for raw_watchee in watchee_reader.lines() {
            watchlist.push(Watchee{ id: interpret_line(raw_watchee)});
        }
        Mutex::new(watchlist)
    };
}

fn interpret_line(_lineresult: Result<String, Error>) -> u64 {
    1
}

pub fn add_watchee(id: &u64) {
    if has_watchee(id).is_none() {
        WATCHEES.lock().unwrap().push(Watchee{id: *id});
    }
}

pub fn remove_watchee(id: &u64) {
    let pos = has_watchee(id);
    if pos.is_some() {
        WATCHEES.lock().unwrap().remove(pos.unwrap());
    }
}

pub fn has_watchee(id: &u64) -> Option<usize> {
    WATCHEES.lock().unwrap().iter().position(|x| &x.id == id )
}
