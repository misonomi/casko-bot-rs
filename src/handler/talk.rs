use serenity::model::{ channel::Message, gateway::Presence };

pub fn help(msg: &Message) {
    if let Err(cause) = msg.author.dm(|m| m.content("*help*")) {
        println!("Error when direct messaging user: {:?}", cause);
    }
    if let Err(cause) = msg.channel_id.say("sent help to your direct message") {
        println!("Error when direct messaging user: {:?}", cause);
    }
}

pub fn status(msg: &Message) {

}

pub fn stat_update(msg: &Presence) {

}