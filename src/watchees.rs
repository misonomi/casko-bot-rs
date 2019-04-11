use std::fs::{ File, OpenOptions };
use std::io::{ BufReader, BufRead, BufWriter, Write, Error };
use std::sync::{ Arc, Mutex, MutexGuard };
use std::path::Path;
use lazy_static::lazy_static;

use serenity::model::{ id::UserId, gateway::Game };

mod stat;
pub mod watchee;
use watchee::Watchee;

lazy_static! {
    static ref WATCHLIST: Arc<Mutex<Vec<Watchee>>> = {
        let mut watchlist = Vec::new();

        if !Path::new("watchees.dat").exists() {
            File::create("watchees.dat").expect("failed to create watchee file");
            println!("created watchees.dat");
        }
        
        let watchee_reader = BufReader::new(OpenOptions::new().read(true).open("watchees.dat").expect("failed to open watchee file"));
        for raw_watchee in watchee_reader.lines() {
            if let Some(watchee) = interpret_line(&raw_watchee) {
                watchlist.push(watchee);
            }
        }
        println!("successflly loaded watchee list");
        Arc::new(Mutex::new(watchlist))
    };
}

fn interpret_line(line: &Result<String, Error>) -> Option<Watchee> {
    // TODO lines().expect() and not match?
    // do when I wont need cause anymore
    match line {
        Ok(line) =>{
            let raw_props: Vec<&str> = line.split(":").collect();
            if raw_props.len() != 2 {
                println!("failed to parse: invalid argc");
                return None;
            }
            let u64id: u64 = raw_props[0].parse().expect("failed to parse userid");
            let u8stat: u8 = raw_props[1].parse().expect("failed to parse stat");
            Some(Watchee::new(UserId::from(u64id), stat::bond_from(u8stat)))
        },
        Err(cause) => {
            println!("Error when reading watchees: {:?}", cause);
            None
        }
    }
}

// TODO propergate result
pub fn get_lock<'a>() -> MutexGuard<'a, Vec<Watchee>> {
    WATCHLIST.lock().expect("failed to obtain lock")
}

pub fn add_watchee(id: &UserId) -> Result<usize, usize> {
    if has_watchee(id).is_none() {
        let mut watchees_guarded = get_lock();
        watchees_guarded.push(Watchee::new(*id, stat::BondType::Normal));
        Ok(watchees_guarded.capacity())
    } else {
        Err(0)
    }
}

pub fn remove_watchee(id: &UserId) -> Result<usize, usize> {
    if let Some(pos) = has_watchee(id) {
        get_lock().remove(pos);
        Ok(pos)
    } else {
        Err(0)
    }
}

pub fn find_watchee<'a>(id: &UserId) -> Watchee {
    let watchlist_guarded = get_lock();
    let list = watchlist_guarded.iter().find(|x| x.id_as_u64() == id.as_u64()).unwrap();
    // FIXME TERRIBLE HORRIBLE NO GOOD VERY BAD HACK
    Watchee::incarnate(*list.id_as_id(), (*list.stat_as_enum()).clone(), list.game_as_option().clone(), *list.timestamp_as_instant())
}

pub fn has_watchee(id: &UserId) -> Option<usize> {
    get_lock().iter().position(|x| x.id_as_u64() == *id.as_u64() )
}

pub fn update_game(target: &Watchee, game: Option<Game>) {
    let mut watchlist_locked = get_lock();
    // FIXME nanimo wakaran help  vvvvvvvv            vvvvvvvvvvvv
    let target = watchlist_locked.iter_mut().find(|x| x == &target).unwrap();
    target.update_game(game);
}

pub fn save() {
    let mut watchee_writer = BufWriter::new(OpenOptions::new().write(true).open("watchees.dat").expect("failed to open watchee file"));
    for watchee in get_lock().iter() {
        watchee_writer.write(format!("{}:{}", watchee.id_as_u64(), watchee.stat_as_u8()).as_bytes()).expect("failed on wirte");
    }
}
