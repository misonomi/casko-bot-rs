#[derive(Debug)]
pub enum BondType {
    normal,
    intimate,
    admin,
    unknown
}

pub fn bond_from(id: u8) -> BondType {
    match id {
        1 => BondType::normal,
        2 => BondType::intimate,
        0 => BondType::admin,
        _ => BondType::unknown
    }
}
