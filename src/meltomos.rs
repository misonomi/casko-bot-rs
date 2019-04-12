use std::fs::{ File, OpenOptions };
use std::time::Instant;
use std::mem;
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
    static ref FILENAME: Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("meltomos.dat")));

    static ref CONTACTS: Arc<Mutex<Vec<Meltomo>>> = {
        let mut contacts = Vec::new();
        let filename = &*FILENAME.lock().expect("failed to lock filename");

        if !Path::new(filename).exists() {
            File::create(filename).expect("failed to create meltomo file");
            println!("created meltomos.dat");
        }
        
        let meltomo_reader = BufReader::new(OpenOptions::new().read(true).open(filename).expect("failed to open meltomo file"));
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
            println!("Error when reading line: {:?}", cause);
            None
        }
    }
}

fn get_lock<'a>() -> MutexGuard<'a, Vec<Meltomo>> {
    CONTACTS.lock().expect("failed to obtain lock")
}

fn find_meltomo<'a>(id: &UserId, contacts_guard: &'a mut MutexGuard<'_, Vec<Meltomo>>) -> Option<&'a mut Meltomo> {
    contacts_guard.iter_mut().find(|x| x.has_id(id))
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

// dont remove in case remove_meltomo revives
pub fn has_meltomo(id: &UserId) -> Option<usize> {
    get_lock().iter().position(|m| m.has_id(id))
}

pub fn conjecture_stat(id: &UserId, stat: BondType) -> bool {
    get_lock().iter().find(|m| m.has_id(id) && m.stat == stat).is_some()
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

pub fn get_stat(id: &UserId) -> Option<BondType> {
    get_lock().iter().find(|m| m.has_id(id)).map(|m| m.stat.clone())
}

pub fn conjecture_seq(id: &UserId, seq: TalkSequence) -> bool {
    get_lock().iter().find(|m| m.has_id(id) && m.seq == seq).is_some()
}

pub fn update_seq(id: &UserId, seq: TalkSequence) {
    if let Some(target) = get_lock().iter_mut().find(|m| m.has_id(id)) {
        target.seq = seq;
    }
}

pub fn conjecture_game(id: &UserId, game: Option<&Game>) -> bool {
    get_lock().iter().find(|m| m.has_id(id) && m.game_changed(game)).is_some()
}

pub fn exchange_game(id: &UserId, game: Option<Game>) -> (Option<Game>, Instant) {
    let mut contacts_locked = get_lock();
    if let Some(target) = contacts_locked.iter_mut().find(|m| m.has_id(id)) {
        (mem::replace(&mut target.game, game), mem::replace(&mut target.last_update, Instant::now()))
    } else {
        (game, Instant::now())
    }
}

pub fn list() {
    for (i, meltomo) in get_lock().iter().enumerate() {
        println!("meltomos No.{}| id:{:?}, status:{:?}, sequence:{:?}, game:{:?}, timestamp:{:?}", 
                i, meltomo.id, meltomo.stat, meltomo.seq, meltomo.game, meltomo.last_update);
    }
}

pub fn save() {
    let filename = &*FILENAME.lock().expect("failed to lock filename");
    let mut meltomo_writer = BufWriter::new(OpenOptions::new().write(true).open(filename).expect("failed to open meltomo file"));
    for meltomo in get_lock().iter() {
        meltomo_writer.write(format!("{}:{}\n", meltomo.id.as_u64(), meltomo.stat.into_borrow()).as_bytes()).expect("failed on wirte");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_root() {
        init();
        test_list();
        test_stat();

        save();
        // look out output file
    }

    fn test_list() {
        test_add_meltomo(1, true);
        test_add_meltomo(2, true);
        test_add_meltomo(3, true);
        test_add_meltomo(1, false);
        test_add_meltomo(100, true);

        test_has_meltomo(1, Some(0));
        test_has_meltomo(3, Some(2));
        test_has_meltomo(0, None);
    }

    fn test_stat() {
        test_watch(2, true);
        test_watch(2, false);
        test_watch(4, true);
        test_conjecture_stat(1, BondType::Normal, true);
        test_conjecture_stat(1, BondType::Watching, false);
        test_conjecture_stat(2, BondType::Watching, true);
        test_conjecture_stat(4, BondType::Watching, true);
        test_conjecture_stat(0, BondType::Normal, false);

        test_unwatch(4, true);
        test_unwatch(1, false);
        test_unwatch(0, false);
        test_conjecture_stat(1, BondType::Normal, true);
        test_conjecture_stat(4, BondType::Normal, true);
        test_conjecture_stat(4, BondType::Watching, false);
        test_conjecture_stat(0, BondType::Normal, false);

        test_get_stat(1, Some(BondType::Normal));
        test_get_stat(2, Some(BondType::Watching));
        test_get_stat(4, Some(BondType::Normal));
        test_get_stat(0, None);
    }

    #[test]
    #[ignore]
    fn test_default_filename() {
        assert_eq!(FILENAME.lock().unwrap().clone(), String::from("meltomos.dat"));
    }

    fn init() {
        {
            let mut filename = FILENAME.lock().unwrap();
            filename.clear();
            filename.push_str("meltomos.dat.test");
        }
        {
            let mut contacts = CONTACTS.lock().unwrap();
            contacts.clear();
        }
    }

    // assert can add -> o = true
    fn test_add_meltomo(i: u64, o: bool) {
        match o {
            true => assert_ne!(add_meltomo(&uid(i)), None),
            false => assert_eq!(add_meltomo(&uid(i)), None),
        }
    }

    fn test_has_meltomo(i: u64, o: Option<usize>) {
        assert_eq!(has_meltomo(&uid(i)), o);
    }

    fn test_watch(i: u64, o: bool) {
        match o {
            true => assert_eq!(watch(&uid(i)), Ok(())),
            false => assert_eq!(watch(&uid(i)), Err(())),
        }
    }

    fn test_unwatch(i: u64, o: bool) {
        match o {
            true => assert_eq!(unwatch(&uid(i)), Ok(())),
            false => assert_eq!(unwatch(&uid(i)), Err(())),
        }
    }

    fn test_conjecture_stat(id: u64, stat: BondType, o: bool) {
        assert_eq!(conjecture_stat(&uid(id), stat), o);
    }

    fn test_get_stat(i: u64, o: Option<BondType>) {
        assert_eq!(get_stat(&uid(i)), o);
    }

    fn uid(num: u64) -> UserId {
        UserId::from(num)
    }
}
