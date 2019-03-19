#[derive(Debug, Clone)]
pub enum BondType {
    Normal,
    Intimate,
    Admin,
    Unknown
}

pub fn bond_from(id: u8) -> BondType {
    match id {
        1 => BondType::Normal,
        2 => BondType::Intimate,
        0 => BondType::Admin,
        _ => BondType::Unknown
    }
}
