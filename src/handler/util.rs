use serenity::model::{ id::ChannelId, user::User };

pub fn dm_facade(user: &User, mes: &str) {
    if let Err(cause) = user.dm(|m| m.content(mes)) {
        println!("Error when direct messaging user: {:?}", cause);
    }
}

pub fn talk_facade(channel: &ChannelId, mes: &str) {
    if let Err(cause) = channel.say(mes) {
        println!("Error when talking: {:?}", cause);
    }
}

