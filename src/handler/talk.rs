use serenity::model::channel::Message;

pub fn help(msg: &Message) {
    if let Err(casuse) = msg.author.dm(|m| m.content("*help*")) {
        println!("Error when direct messaging user: {:?}", casuse);
    }
    if let Err(casuse) = msg.channel_id.say("sent help to your direct message") {
        println!("Error when direct messaging user: {:?}", casuse);
    }
}

pub fn status(msg: &Message) {

}