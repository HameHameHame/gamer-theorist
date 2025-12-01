use std::io;
use std::io::Write;
use crate::gamespace::*;


pub struct App {
    pub settings: Settings,
    pub gamespace: Option<Gamespace>,
}

impl App {
    pub fn new() -> Self {
        Self {
            settings: Settings::default(),
            gamespace: None,
        }
    }
    pub fn main_menu(&mut self) {
        clear();
        println!("  MAIN MENU");
        println!("--=========--");
        println!("1) Start Simulation");
        println!("2) Simulation Settings");
        println!("3) Exit");
        println!();
        print!("select option: ");
        io::stdout().flush().unwrap();
        self.main_menu_selection(get_user_int());
    }
    fn main_menu_selection(&mut self,sel: usize) {
        match sel {
            1 => self.simulation_start(),
            2 => self.settings_menu(),
            3 => quit_program(),
            _ => self.main_menu(),
        }
    }
    fn settings_menu(&mut self) {
        clear();
        println!("  SIMULATION SETTINGS");
        println!("--===================--");
        println!("1) world width: {}", self.settings.world_width);
        println!("2) world height: {}", self.settings.world_height);
        println!("3) population percentage: {}", self.settings.starting_pop_percentage);
        println!("9) go back <");
        println!();
        println!("select option: ");
        io::stdout().flush().unwrap();
        self.settings_menu_selection(get_user_int());
    }
    fn settings_menu_selection(&mut self, sel: usize){
        match sel {
            1 => self.set_world_width(),
            2 => self.set_world_height(),
            3 => self.set_pop_percentage(),
            9 => self.main_menu(),
            _ => self.settings_menu(),
        }
    }

    fn set_world_width(&mut self) {
        println!("Enter world width: ");
        io::stdout().flush().unwrap();
        self.settings.world_width = get_user_int();
        self.settings_menu();
    }

    fn set_world_height(&mut self) {
        println!("Enter world height: ");
        io::stdout().flush().unwrap();
        self.settings.world_height = get_user_int();
        self.settings_menu();
    }

    fn set_pop_percentage(&mut self) {
        println!("Enter starting population percentage: ");
        io::stdout().flush().unwrap();
        self.settings.starting_pop_percentage = get_user_int();
        self.settings_menu();
    }
    fn simulation_start(&mut self) {
        clear();
        println!("simulation starting!");
        self.gamespace = Some(Gamespace::new(&self.settings));
        self.gamespace.as_mut().expect("gamespace not made").play();
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

fn clear() {
    print!("\x1B[2J\x1B[H");
}

fn quit_program() {
    std::process::exit(0)
}

#[derive(Debug)]
pub struct Settings {
    pub world_width: usize,
    pub world_height: usize,
    pub starting_pop_percentage: usize
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            world_width: 30,
            world_height: 30,
            starting_pop_percentage: 8,
        }
    }
}