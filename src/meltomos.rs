use std::fs::{ File, OpenOptions };
use std::io::{ BufReader, BufRead, BufWriter, Write, Error };
use std::sync::{ Arc, Mutex, MutexGuard };
use std::path::Path;
use lazy_static::lazy_static;

use serenity::model::{ id::UserId, gateway::Game };

mod stat;
pub mod meltomo;
use meltomo::Meltomo;

lazy_static! {
    static ref WATCHLIST: Arc<Mutex<Vec<Meltomo>>> = {
        let mut watchlist = Vec::new();

        if !Path::new("meltomos.dat").exists() {
            File::create("meltomos.dat").expect("failed to create meltomo file");
            println!("created meltomos.dat");
        }
        
        let meltomo_reader = BufReader::new(OpenOptions::new().read(true).open("meltomos.dat").expect("failed to open meltomo file"));
        for raw_meltomo in meltomo_reader.lines() {
            if let Some(meltomo) = interpret_line(&raw_meltomo) {
                watchlist.push(meltomo);
            }
        }
        println!("successflly loaded meltomo list");
        Arc::new(Mutex::new(watchlist))
    };
}

fn interpret_line(line: &Result<String, Error>) -> Option<Meltomo> {
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
            Some(Meltomo::new(UserId::from(u64id), stat::bond_from(u8stat)))
        },
        Err(cause) => {
            println!("Error when reading meltomos: {:?}", cause);
            None
        }
    }
}

// TODO propergate result
pub fn get_lock<'a>() -> MutexGuard<'a, Vec<Meltomo>> {
    WATCHLIST.lock().expect("failed to obtain lock")
}

pub fn add_meltomo(id: &UserId) -> Result<usize, usize> {
    if has_meltomo(id).is_none() {
        let mut meltomos_guarded = get_lock();
        meltomos_guarded.push(Meltomo::new(*id, stat::BondType::Normal));
        Ok(meltomos_guarded.capacity())
    } else {
        Err(0)
    }
}

pub fn remove_meltomo(id: &UserId) -> Result<usize, usize> {
    if let Some(pos) = has_meltomo(id) {
        get_lock().remove(pos);
        Ok(pos)
    } else {
        Err(0)
    }
}

pub fn find_meltomo(id: &UserId) -> Meltomo {
    let watchlist_guarded = get_lock();
    // FIXME TERRIBLE HORRIBLE NO GOOD VERY BAD HACK
    watchlist_guarded.iter().find(|x| x.id_as_u64() == id.as_u64()).unwrap().incarnate()
}

pub fn has_meltomo(id: &UserId) -> Option<usize> {
    get_lock().iter().position(|x| x.id_as_u64() == id.as_u64() )
}

pub fn update_game(target: &Meltomo, game: Option<Game>) {
    let mut watchlist_locked = get_lock();
    // FIXME nanimo wakaran help  vvvvvvvv            vvvvvvvvvvvv
    let target = watchlist_locked.iter_mut().find(|x| x == &target).unwrap();
    target.update_game(game);
}

pub fn save() {
    let mut meltomo_writer = BufWriter::new(OpenOptions::new().write(true).open("meltomos.dat").expect("failed to open meltomo file"));
    for meltomo in get_lock().iter() {
        meltomo_writer.write(format!("{}:{}", meltomo.id_as_u64(), meltomo.stat_as_u8()).as_bytes()).expect("failed on wirte");
    }
}
