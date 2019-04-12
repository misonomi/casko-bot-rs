use serenity::model::{ channel::Message };

use super::util::{ talk_facade, react_facade };
use crate::meltomos;
use crate::meltomos::stat::TalkSequence;

#[derive(Debug, Clone)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

pub fn choose(msg: &Message, diffic: Difficulty) -> bool {
    if !meltomos::conjecture_seq(&msg.author.id, TalkSequence::ChooseDiffic) { return false; }
    match diffic {
        Difficulty::Easy => {
            react_facade(msg, "✅");
            talk_facade(&msg.channel_id, "");
            meltomos::update_seq(&msg.author.id, TalkSequence::ChooseDiffic);
        },
        Difficulty::Normal => {
            react_facade(msg, "✅");
            talk_facade(&msg.channel_id, "");
            meltomos::update_seq(&msg.author.id, TalkSequence::ChooseDiffic);
        },
        Difficulty::Hard => {
            react_facade(msg, "✅");
            talk_facade(&msg.channel_id, "");
            meltomos::update_seq(&msg.author.id, TalkSequence::ChooseDiffic);
        },
    }
    true
}
