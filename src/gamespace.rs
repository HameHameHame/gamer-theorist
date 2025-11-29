use crate::entity::*;
use crate::menu::*;

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
    pub fn new(settings: Settings) -> Self {
        Self {
            world: World::new(settings.world_width, settings.world_height),
            entities: self.generate_population(settings),
        }
    }

    fn make_empty_world(&mut self, settings: Settings) {
        self.world = World {
            width: settings.world_width,
            height: settings.world_height,
            tiles: self.make_grid(settings.world_height, settings.world_width)
        }
    }
    // fn make_grid(&self, rows: usize, cols: usize) -> Vec<Vec<Tile>> {
    //     let mut grid = Vec::new();   
    //     for _r in 0..rows {
    //         let mut row = Vec::new();
    //         for _c in 0..cols {
    //             row.push(Tile {occupant: None, tiletype: TileType::Grass});
    //         }
    //         grid.push(row);
    //     }
    //     return grid;
    // }

    // fn make_empty_world(cols: usize, rows: usize) -> World {
    //     let empty_world = World {width: cols, height: rows, tiles: make_grid(rows,cols)};
    //     return empty_world
    // }
}
