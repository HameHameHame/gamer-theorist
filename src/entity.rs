use crate::gamespace::*;
pub enum EntityType {
    Hawk,
    Dove,
}

pub enum Direction {
    North,
    East,
    South,
    West
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

    pub fn move_step(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.posxy = (self.posxy.0, self.posxy.1 - 1),
            Direction::East => self.posxy = (self.posxy.0 + 1, self.posxy.1),
            Direction::South => self.posxy = (self.posxy.0, self.posxy.1 + 1), 
            Direction::West => self.posxy = (self.posxy.0 - 1, self.posxy.1),
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
