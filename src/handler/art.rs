use std::fs::{ File, OpenOptions };
use std::io::{ BufReader, BufRead };
use std::path::Path;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;

use serenity::model::channel::Message;

use super::util::{ talk_facade, react_facade };


#[macro_use(lazy_static)]
lazy_static! {
    static ref ARTIDLIST: Vec<String> = {
        let mut artlist = Vec::new();
        let filename = "arts.dat";

        if !Path::new(filename).exists() {
            File::create(filename).expect("failed to create arts file");
            println!("created arts.dat");
        }
        
        let artid_reader = BufReader::new(OpenOptions::new().read(true).open(filename).expect("failed to open arts file"));
        for raw_artid in artid_reader.lines() {
            raw_artid.map(|x| artlist.push(x)).expect("failed to parse line");
        }
        println!("successflly loaded art list");
        artlist
    };
}

pub fn random(msg: &Message) -> bool {
    react_facade(msg, "🎨");
    talk_facade(&msg.channel_id, &*format!("http://{}", ARTIDLIST.choose(&mut rand::thread_rng()).unwrap_or(&String::from("0"))));
    true
}
