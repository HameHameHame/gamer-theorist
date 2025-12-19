use std::io;
use std::io::Write;
use rand::{rng, seq::SliceRandom};

mod entity;
mod gamespace;
mod render;
mod menu;
use menu::*;
use entity::*;
use gamespace::*;
use render::*;

println!("push test");
// fn get_tile_render_char (tile: &Tile) -> char {
//     match tile.tiletype {
//         TileType::Empty => '+',
//         TileType::Grass => 'G',
//         TileType::Rock => 'R',
//     }
// }

// fn get_entity_by_id(id: EntityID) -> Entity{
//     entity_storage[id]
// }

// fn get_entity_render_char (tile: &Tile) -> char {
//     get_entity_by_id(tile.occupant);
//     match tile.occupant {
//         None => 'E',
//         Hawk => 'H',
//         Dove => 'D',
//     }
// }

// fn get_render_char (tile: &Tile) -> char{
//     if tile.entity_id == 0 {
//         return get_tile_render_char(&tile)
//     }
//     get_entity_render_char(&tile)
// }

// fn print_char(pixel: char) {
//     let mut output = io::stdout();
//     let mut bufr = [0; 4];
//     let f = pixel.encode_utf8(&mut bufr);
//     output.write_all(f.as_bytes()).unwrap();
// }

// fn populate_grid(grid: &mut Vec<Vec<Tile>>) {
//     let width = grid[0].len();
//     let height = grid.len();
//     let total_space = height * width;
//     let seeds = unique_rand_nums(total_space, 15);
//     for entity in seeds {
//         //grid[entity / width][entity % width].occupant = EntityType::Dove;
//         println!("fix later")
//     }
// }

// fn unique_rand_nums(max: usize, percent: usize) -> Vec<usize> {
//     println!("running");
//     let mut nums: Vec<_> = (0..max).collect();
//     let mut rng = rng();
//     nums.shuffle(&mut rng);
//     let count = max * percent / 100;
//     nums.truncate(count);
//     nums
// }

// fn render_grid(grid: &Vec<Vec<Tile>>) {
//     for row in grid {
//         for value in row {
//             print_char(get_render_char(value));
//         }
//         print_char('\n');
//     }
// }

// fn get_user_input() -> String {
//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer).expect("fucked it");
//     buffer.trim().to_string()
// }

// fn get_user_int() -> usize{
//     get_user_input().parse().expect("not number")
// }

// fn make_grid(rows: usize, cols: usize) -> Vec<Vec<Tile>> {
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

// fn make_entity_storage() -> Vec<Entity> {
//     let entity_storage = Vec::new();
//     return entity_storage
// }

fn main() {
    //let rows = get_user_int();
    //let cols = get_user_int();
    //let gamespace = Simulation{world: make_empty_world(cols, rows),entities: make_entity_storage()};
    let mut app = App::new();
    app.main_menu();
}
