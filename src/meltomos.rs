use std::fs::{ File, OpenOptions };
use std::io::{ BufReader, BufRead, BufWriter, Write, Error };
use std::sync::{ Arc, Mutex, MutexGuard };
use std::path::Path;
use lazy_static::lazy_static;

use serenity::model::{ id::UserId, gateway::Game };

pub mod stat;
use stat::{ BondType, TalkSequence };
pub mod meltomo;
use meltomo::Meltomo;

lazy_static! {
    static ref CONTACTS: Arc<Mutex<Vec<Meltomo>>> = {
        let mut contacts = Vec::new();

        if !Path::new("meltomos.dat").exists() {
            File::create("meltomos.dat").expect("failed to create meltomo file");
            println!("created meltomos.dat");
        }
        
        let meltomo_reader = BufReader::new(OpenOptions::new().read(true).open("meltomos.dat").expect("failed to open meltomo file"));
        for raw_meltomo in meltomo_reader.lines() {
            if let Some(meltomo) = interpret_line(&raw_meltomo) {
                contacts.push(meltomo);
            }
        }
        println!("successflly loaded meltomo list");
        Arc::new(Mutex::new(contacts))
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
            Some(Meltomo::new(UserId::from(u64id), BondType::from(u8stat)))
        },
        Err(cause) => {
            println!("Error when reading meltomos: {:?}", cause);
            None
        }
    }
}

pub fn get_lock<'a>() -> MutexGuard<'a, Vec<Meltomo>> {
    CONTACTS.lock().expect("failed to obtain lock")
}

pub fn add_meltomo(id: &UserId) -> Option<usize> {
    if has_meltomo(id).is_none() {
        let mut contacts = get_lock();
        contacts.push(Meltomo::new(*id, stat::BondType::Normal));
        Some(contacts.capacity())
    } else {
        None
    }
}

pub fn watch(id: &UserId) -> Result<(), ()> {
    let mut contacts_guard = get_lock();
    if let Some(target) = find_meltomo(id, &mut contacts_guard) {
        target.change_stat(BondType::Watching)
    } else {
        contacts_guard.push(Meltomo::new(*id, stat::BondType::Watching));
        Ok(())
    }
}

pub fn unwatch(id: &UserId) -> Result<(), ()> {
    let mut contacts_guard = get_lock();
    if let Some(target) = find_meltomo(id, &mut contacts_guard) {
        target.change_stat(BondType::Normal)
    } else {
        Err(())
    }
}

pub fn find_meltomo<'a>(id: &UserId, contacts_guarded: &'a mut MutexGuard<'_, Vec<Meltomo>>) -> Option<&'a mut Meltomo> {
    contacts_guarded.iter_mut().find(|x| x.has_id(id))
}

pub fn has_meltomo(id: &UserId) -> Option<usize> {
    get_lock().iter().position(|x| x.has_id(id))
}

pub fn is_seq(id: &UserId, seq: TalkSequence) -> bool {
    get_lock().iter().find(|x| x.has_id(id) && x.seq == seq).is_some()
}

pub fn update_seq(id: &UserId, seq: TalkSequence) {
    if let Some(target) = get_lock().iter_mut().find(|x| x.has_id(id)) {
        target.seq = seq;
    }
}

pub fn update_game(id: &UserId, game: Option<Game>) {
    let mut contacts_locked = get_lock();
    if let Some(target) = contacts_locked.iter_mut().find(|x| x.has_id(id)) {
        target.update_game(game);
    }
}

pub fn save() {
    let mut meltomo_writer = BufWriter::new(OpenOptions::new().write(true).open("meltomos.dat").expect("failed to open meltomo file"));
    for meltomo in get_lock().iter() {
        meltomo_writer.write(format!("{}:{}", meltomo.id.as_u64(), meltomo.stat.into_borrow()).as_bytes()).expect("failed on wirte");
    }
}
