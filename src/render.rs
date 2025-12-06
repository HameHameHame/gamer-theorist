use crate::gamespace::*;
use crate::entity::*;
use std::io;
use std::io::Write;


pub fn render_world(gamespace: &Gamespace) {
    for row in &gamespace.world.tiles {
        for tile in row {
            match tile.occupant {
                Some(_) => render_entity(&gamespace.entities[tile.occupant.unwrap()]),
                None => render_space(&tile),
            }
        }
        print!("\n");
    }
}

fn render_entity(entity: &Entity) {
    print_char(get_entity_char(entity))
}

fn get_entity_char(entity: &Entity) -> char {
    match entity.species {
        EntityType::Hawk => 'H',
        EntityType::Dove => 'D',
    }
}

fn render_space(tile: &Tile) {
    print_char(get_space_char(tile))
}

fn get_space_char(tile: &Tile) -> char{
    match tile.tiletype {
        TileType::Empty => '+',
        TileType::Grass => '|',
        TileType::Rock => '=',
    }
}

fn print_char(pixel: char) {
    let mut output = io::stdout();
    let mut bufr = [0; 4];
    let f = pixel.encode_utf8(&mut bufr);
    output.write_all(f.as_bytes()).unwrap();
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}