use std::u8::MAX;

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

pub fn bond_to(bond: &BondType) -> u8 {
    match bond {
        BondType::Normal => 1,
        BondType::Intimate => 2,
        BondType::Admin => 0,
        BondType::Unknown => MAX
    }
}
