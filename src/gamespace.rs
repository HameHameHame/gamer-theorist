use crate::entity;
use crate::entity::*;
use crate::menu::*;
use crate::render::*;
use rand::{rng, seq::SliceRandom};


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
    pub fn place_entity_in_world(&mut self, id: EntityID, x: usize, y: usize) {
        println!("world place id is currently {}", id);
        self.tiles[y][x].occupant = Some(id);
        println!("and the value after setting = {:?}", self.tiles[y][x].occupant)
    }

    pub fn move_entity(&mut self, id: EntityID, old_posxy: (usize, usize), new_posxy: (usize, usize) ) {
        self.tiles[old_posxy.1][old_posxy.0].occupant = None;
        self.tiles[new_posxy.1][new_posxy.0].occupant = Some(id);
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
    pub fn play(&mut self) {
        for ent in &self.entities {
            println!("----------");
            println!("entity id: {}", ent.entity_id);
            println!("entity hp: {}", ent.hp);
        }
        println!("total entities = {}", &self.entities.len());
        println!("printable? -- {}", self.rng_entity_positions()[0]);
        println!("worldstat: {}", self.world.width);
        self.populate_world();
        println!("entity id: {}", self.entities[0].entity_id);
        println!("entity position be random = {:?}", self.entities[0].posxy);
        println!("tilepos be matching with id: {:?}", self.world.tiles[self.entities[0].posxy.1][self.entities[0].posxy.0].occupant);
        clear_screen();
        self.game_loop();
    }

    fn game_loop(&mut self) {
        for _ in 0..200 {
            clear_screen();
            render_world(self);
            self.simulation_step();
            std::thread::sleep(std::time::Duration::from_millis(333));
        }
    }

    fn simulation_step (&mut self) {
        println!("self ent len = {}", self.entities.len());
        for entity_id in 0..self.entities.len() {
            let new_move = self.entities[entity_id].request_random_move();
            if self.parse_move_request(new_move, self.entities[entity_id].posxy) {
                let old_pos = self.entities[entity_id].posxy;
                self.entities[entity_id].move_step(new_move);
                self.world.move_entity(entity_id, old_pos, self.entities[entity_id].posxy);
            }
        }
    }

    fn parse_move_request (&mut self,direction: Direction, currentpos:(usize, usize)) -> bool {
        let desired_pos = new_pos_calc(direction, currentpos);
        if self.pos_in_bounds(desired_pos) && self.world.tiles[desired_pos.1][desired_pos.0].occupant == None {
            return true
        }
        return false
    }

    fn pos_in_bounds(&self, posxy: (usize, usize)) -> bool {
        posxy.0 < self.world.width - 1 && posxy.1 < self.world.height - 1
    }


    pub fn populate_world(&mut self) {
        let seeds = self.rng_entity_positions();
        for (count, entity_pos) in seeds.iter().enumerate() {
            self.place_entity(count, *entity_pos);
        }
    }

    fn place_entity(&mut self,id: EntityID, pos: usize) {
        let posxy = (pos % self.world.width, pos / self.world.width);
        self.world.place_entity_in_world(id, posxy.0, posxy.1);
        self.entities[id].posxy = posxy;
    }

    pub fn rng_entity_positions(&self) -> Vec<usize> {
        let mut pos_list: Vec<_> = (0..total_space(self.world.width, self.world.height)).collect();
        let mut rng = rng();
        pos_list.shuffle(&mut rng);
        pos_list.truncate(self.entities.len());
        pos_list
    }
}

fn total_starting_entities(percent: usize, maxspace: usize) -> usize {
    maxspace * percent / 100
}

fn total_space(width: usize, height: usize) -> usize {
    width * height
}

fn new_pos_calc(direction: Direction, currentpos:(usize, usize)) -> (usize, usize) {
    match direction {
            Direction::North => (currentpos.0, currentpos.1.saturating_sub(1)),
            Direction::East => (currentpos.0 + 1, currentpos.1),
            Direction::South => (currentpos.0, currentpos.1 + 1), 
            Direction::West => (currentpos.0.saturating_sub(1), currentpos.1),
        }
}
