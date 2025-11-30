use crate::entity::*;
use crate::menu::*;

pub type EntityID = usize;
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

impl World{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            tiles: Self::make_grid(height, width),
        }
    }
    fn make_grid(rows: usize, cols: usize) -> Vec<Vec<Tile>> {
        let mut grid = Vec::new();   
        for _r in 0..rows {
            let mut row = Vec::new();
            for _c in 0..cols {
                row.push(Tile {occupant: None, tiletype: TileType::Grass});
            }
            grid.push(row);
        }
        return grid;
    }
}

pub struct Gamespace {
    pub world: World,
    pub entities: Vec<Entity>,
}

impl Gamespace {
    pub fn new(settings: &Settings) -> Self {
        Self {
            world: World::new(settings.world_width, settings.world_height),
            entities: Gamespace::populate_entities(settings.starting_pop_percentage, settings.world_width, settings.world_height),
        }
    }
    pub fn populate_entities(perc: usize, width: usize, height: usize) -> Vec<Entity> {
        let mut entities: Vec<Entity> = Vec::new();
        for id in 0..total_starting_entities(perc, total_space(width, height)) {
            entities.push(Entity::new(id, EntityType::Dove))
        }
        return entities
    }
    pub fn play(&self) {
        for ent in &self.entities {
            println!("----------");
            println!("entity id: {}", ent.entity_id);
            println!("entity hp: {}", ent.hp);
        }
        println!("total entities = {}", &self.entities.len())
    }
}

fn total_starting_entities(percent: usize, maxspace: usize) -> usize {
    maxspace * percent / 100
}

fn total_space(width: usize, height: usize) -> usize {
    width * height
}
