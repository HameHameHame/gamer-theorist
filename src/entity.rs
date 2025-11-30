use crate::gamespace::*;
pub enum EntityType {
    Hawk,
    Dove,
}

pub struct Entity {
    pub entity_id: EntityID,
    pub species: EntityType,
    pub posxy: (usize, usize),
    pub hp: u32,
}

impl Entity {
    pub fn new(id: EntityID, species: EntityType) -> Self {
        match species {
            EntityType::Dove => Self::make_dove(id),
            EntityType::Hawk => Self::make_hawk(id),
            _ => Self::make_dove(id),
        }
    }
    fn make_dove(id: EntityID) -> Self {
        Self {
            entity_id: id,
            species: EntityType::Dove,
            posxy: (0,0),
            hp: 100,
        }
    }
    fn make_hawk(id: EntityID) -> Self {
        Self {
            entity_id: id,
            species: EntityType::Hawk,
            posxy: (0,0),
            hp: 150,
        }
    }
}
