use crate::gamespace::*;


pub fn render_world(world: &World) {
    for row in &world.tiles {
        for tile in row {
            render_tile(&tile);
        }
        print!("\n");
    }
}

fn render_tile(tile: &Tile) {
    match Some(tile.occupant) {
        Some(_) => render_entity(&tile),
        None => render_space(&tile),
    }
}

fn render_entity() {

}

fn render_space() {
    
}