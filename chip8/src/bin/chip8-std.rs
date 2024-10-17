use core::str;
use std::{
    cell::RefCell,
    thread,
    time::{Duration, SystemTime},
};

pub const IBM_LOGO: &[u8] = include_bytes!("../../../chip8-roms/roms/IBM Logo.ch8");

fn main() {
    let mut app = chip8::app::AppState::new(IBM_LOGO);
    let mut d = RefCell::new(app.display);

    loop {
        //let display = RefCell::clone(&app.display);
        
        app.step();
        
        for i in d.get_mut() {
            match i {
                chip8::display::DisplayStates::On => {
                    print!("X");
                }
                chip8::display::DisplayStates::Off => {
                    print!("_");
                }
                chip8::display::DisplayStates::NewLine => {
                    println!("");
                }
            }
        }
        
        std::thread::sleep(Duration::new(1, 0));
    }

    //ufmt::uwriteln!(&mut serial, "Hello from ATmega!\r").unwrap();
}
