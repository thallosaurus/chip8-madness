use std::{thread, time::Duration};

use chip8::{app::AppState, chip8::ch8_types::DISPLAY_WIDTH};

//pub const PRG: &[u8] = include_bytes!("../../chip8-roms/roms/IBM Logo.ch8");
pub const PRG: &[u8] = include_bytes!("../../chip8-test-rom/test_opcode.ch8");

fn main() {
    let mut rt = AppState::new(PRG);
    let mut exit = false;
    //let mut i = 0;
    loop {
        if let Err(data) = rt.step() {
            println!("unknown opcode: {}", data);
            return;
        }

        print_to_console(&rt);
        
        thread::sleep(Duration::from_millis(10));
    }
}

fn print_to_console(rt: &AppState) {
    let output: Vec<String> = rt.vram.iter().map(|f| {
        row_to_string(&f)
    }).collect();

    println!("{}", output.join(""));
}

pub fn row_to_string(o: &[bool; DISPLAY_WIDTH]) -> String {
    let mut s = String::new();

    let mut i = 7;
    while i < o.len() {
        s.push(if o[i] { 'X' } else { '_' });
        i += 1;
    }

    s.push('\n');

    s
}