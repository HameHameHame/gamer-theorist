
pub enum EntityType {
    Hawk,
    Dove,
}

pub struct Entity{
    id: u32,
    species: EntityType,
    posxy: (usize, usize),
    hp: u32,
}