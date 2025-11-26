use std::io;
use std::io::Write;
use rand::{rng, seq::SliceRandom};

fn make_grid(rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut grid = Vec::new();   
    for _r in 0..rows {
        let mut row = Vec::new();
        for _c in 0..cols {
            row.push('.')
        }
        grid.push(row);
    }
    return grid;
}

fn print_char(pixel: char) {
    let mut output = io::stdout();
    let mut bufr = [0; 4];
    let f = pixel.encode_utf8(&mut bufr);
    output.write_all(f.as_bytes()).unwrap();
}

fn populate_grid(grid: &mut Vec<Vec<char>>) {
    let width = grid[0].len();
    let height = grid.len();
    let total_space = height * width;
    let seeds = unique_rand_nums(total_space, 15);
    for entity in seeds {
        grid[entity / width][entity % width] = '0';
    }
}

fn unique_rand_nums(max: usize, percent: usize) -> Vec<usize> {
    println!("running");
    let mut nums: Vec<_> = (0..max).collect();
    let mut rng = rng();
    nums.shuffle(&mut rng);
    let count = max * percent / 100;
    nums.truncate(count);
    println!("{}", nums[0]);
    nums
}

fn render_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for value in row {
            print_char(*value);
        }
        print_char('\n');
    }
}

fn get_user_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("fucked it");
    buffer.trim().to_string()
}

fn get_user_int() -> usize{
    get_user_input().parse().expect("not number")
}

fn place_character(grid: &mut Vec<Vec<char>>,entity: char, xpos: usize, ypos: usize) {
    grid[xpos][ypos] = entity;
}

//fn get_pos () -> (usize, usize) {
//
//}

fn main() {
    let rows = get_user_int();
    let cols = get_user_int();
    let mut user_grid = make_grid(rows, cols);
    render_grid(&user_grid);
    populate_grid(&mut user_grid);
    println!("--------");
    render_grid(&user_grid);
    println!("--------");
    place_character(&mut user_grid, 'A', rows / 2 - 1, cols / 2 - 1);
    render_grid(&user_grid);
    println!("col = {}", user_grid.len());
    println!("row = {}", user_grid[0].len());
    
}
