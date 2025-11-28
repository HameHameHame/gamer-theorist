use std::io;
use std::io::Write;


fn get_user_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("fucked it");
    buffer.trim().to_string()
}

fn get_user_int() -> usize{
    get_user_input().parse().expect("not number")
}

pub fn main_menu() {
    println!{"  MAIN MENU"};
    println!{"--=========--"};
    println!{"1) Start Simulation"};
    println!{"2) Simulation Settings"};
    println!{"3) Exit"};
    println!{""}
    print!{"select option: "}
    io::stdout().flush().unwrap();
    let selection = get_user_int();

    main_menu_selection(selection);
}

fn main_menu_selection(sel: usize) {
    match sel {
        1 => simulation_start(),
        2 => settings_menu(),
        3 => quit_program(),
        _ => main_menu(),
    }
}

fn simulation_start() {
    println!("simulation start")
}

fn settings_menu() {
    println!("settings menu")
}

fn quit_program() {
    std::process::exit(0)
}