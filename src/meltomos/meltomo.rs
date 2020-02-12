use std::time::Instant;

use serenity::model::{ id::UserId, gateway::Activity };

use super::stat::{ BondType, TalkSequence };

pub struct Meltomo {
    pub id: UserId,
    pub stat: BondType,
    pub seq: TalkSequence,
    pub game: Option<Activity>,
    pub last_update: Instant
}

impl Meltomo {
    pub fn new(id: UserId, stat: BondType) -> Meltomo {
        Meltomo{ 
            id: id, 
            stat: stat, 
            seq: TalkSequence::None, 
            game: None, 
            last_update: Instant::now(),
        }
    }

    pub fn has_id(&self, id: &UserId) -> bool {
        self.id.as_u64() == id.as_u64()
    }

    pub fn change_stat(&mut self, stat: BondType) -> Result<(), ()> {
        if self.stat == stat {
            Err(())
        } else {
            self.stat = stat;
            Ok(())
        }
    }

    pub fn game_changed(&self, new_game: Option<&Activity>) -> bool {
        match (self.game.as_ref(), new_game) {
            (Some(old), Some(new)) => old.name != new.name,
            (None, None) => false,
            _ => true,
        }
    }
}

impl PartialEq for Meltomo {
    fn eq(&self, target: &Meltomo) -> bool {
        self.id.as_u64() == target.id.as_u64()
        && self.stat == target.stat
        && self.seq == target.seq
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_id() {
        let meltomo = Meltomo::new(UserId::from(123), BondType::Normal);
        assert!(meltomo.has_id(&UserId::from(123)));
        assert!(!meltomo.has_id(&UserId::from(12)));
        assert!(!meltomo.has_id(&UserId::from(124)));
    }

    #[test]
    fn test_change_stat() {
        let mut meltomo = Meltomo::new(UserId::from(123), BondType::Normal);
        assert_eq!(meltomo.change_stat(BondType::Watching), Ok(()));
        assert_eq!(meltomo.change_stat(BondType::Watching), Err(()));
        assert_eq!(meltomo.change_stat(BondType::Admin), Ok(()));
    }

    #[test]
    fn test_game_changed() {
        let mut meltomo = Meltomo::new(UserId::from(123), BondType::Normal);
        assert!(!meltomo.game_changed(None));
        assert!(meltomo.game_changed(Some(&Activity::playing("Fate/EXTRA"))));
        // should idempotent
        assert!(!meltomo.game_changed(None));
        assert!(meltomo.game_changed(Some(&Activity::playing("Fate/EXTRA"))));

        meltomo.game = Some(Activity::playing("Fate/EXTRA"));
        assert!(meltomo.game_changed(None));
        assert!(!meltomo.game_changed(Some(&Activity::playing("Fate/EXTRA"))));
        assert!(meltomo.game_changed(Some(&Activity::playing("Fate/EXTRA CCC"))));
        assert!(meltomo.game_changed(Some(&Activity::playing("Armored Core 4"))));
    }

    #[test]
    fn test_eq() {
        let (meltomo_a, mut meltomo_b) = generate_double_meltomo();
        assert!(meltomo_a == meltomo_b);
        meltomo_b.id = UserId::from(999);
        assert!(meltomo_a != meltomo_b);
        meltomo_b.stat = BondType::Admin;
        assert!(meltomo_a != meltomo_b);
        meltomo_b.seq = TalkSequence::ChooseDiffic;
        assert!(meltomo_a != meltomo_b);

        let (mut meltomo_a, mut meltomo_b) = generate_double_meltomo();
        meltomo_a.stat = BondType::Watching;
        assert!(meltomo_a != meltomo_b);
        meltomo_b.seq = TalkSequence::FreeTalk;
        assert!(meltomo_a != meltomo_b);

        let (mut meltomo_a, meltomo_b) = generate_double_meltomo();
        meltomo_a.seq = TalkSequence::FreeTalk;
        assert!(meltomo_a != meltomo_b);

        let (meltomo_a, mut meltomo_b) = generate_double_meltomo();
        meltomo_b.game = Some(Activity::playing("Fate/EXTRA"));
        assert!(meltomo_a == meltomo_b);
    }

    fn generate_double_meltomo() -> (Meltomo, Meltomo) {
        (Meltomo::new(UserId::from(123), BondType::Normal), Meltomo::new(UserId::from(123), BondType::Normal))
    }
}
