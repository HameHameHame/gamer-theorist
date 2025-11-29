use crate::entity::*;

pub type EntityID = u32;
pub struct Tile {
    pub occupant: Option<EntityID>,
    pub tiletype: TileType
}

pub enum TileType {
    Empty,
    Grass,
    Rock
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
}

pub struct Gamespace {
    pub world: World,
    pub entities: Vec<Entity>,
}
