use std::{thread, time::Duration};

use chip8::{app::AppState, chip8::ch8_types::DISPLAY_WIDTH};

#[cfg(debug_assertions)]
const PIXEL_ON: char = 'X';

#[cfg(debug_assertions)]
const PIXEL_OFF: char = '_';

#[cfg(not(debug_assertions))]
const PIXEL_ON: char = '◼';

#[cfg(not(debug_assertions))]
const PIXEL_OFF: char = ' ';

pub fn print_to_string(rt: &AppState) -> String {
    let output: Vec<String> = rt.vram.iter().map(|f| {
        row_to_string(&f)
    }).collect();
    output.join("")
}

pub fn print_to_console(rt: &AppState) {
    println!("{}", print_to_string(rt));
}

fn row_to_string(o: &[bool; DISPLAY_WIDTH]) -> String {
    let mut s = String::new();

    let mut i = 0;
    while i < o.len() {
        s.push(if o[i] { PIXEL_ON } else { PIXEL_OFF });
        i += 1;
    }

    s.push('\n');

    s
}

pub fn run_rom(prog: &[u8]) {
    let mut rt = AppState::new(prog);
    loop {
        if let Err(data) = rt.step() {
            println!("unknown opcode: {}", data);
            break;
        }

        rt.dec_timers();
        print_to_console(&rt);
        
        thread::sleep(Duration::from_millis(10));
    }
}

pub fn iterate_rom(prog: &[u8], frames: usize) -> String {
    let mut rt = AppState::new(prog);

    let mut i = 0;
    while i < frames {
        if let Err(data) = rt.step() {
            println!("unknown opcode: {}", data);
            break;
        }
        rt.dec_timers();
        i += 1;
    }
    print_to_string(&rt)
}

#[cfg(test)]
mod rom_tests {
    use crate::iterate_rom;

    #[test]
    fn chip8_logo() {
        pub const PRG: &[u8] = include_bytes!("../../chip8-test-suite/bin/1-chip8-logo.ch8");
        let left = iterate_rom(PRG, 40);
        
        let right = include_str!("1-chip8-logo.txt");
        assert_eq!(left.as_str(), right)
    }

    #[test]
    fn ibm_logo() {
        pub const PRG: &[u8] = include_bytes!("../../chip8-test-suite/bin/2-ibm-logo.ch8");
        let left = iterate_rom(PRG, 20);
        
        let right = include_str!("2-ibm-logo.txt");
        assert_eq!(left.as_str(), right)
    }

    #[test]
    fn corax() {
        pub const PRG: &[u8] = include_bytes!("../../chip8-test-suite/bin/3-corax+.ch8");
        let left = iterate_rom(PRG, 1000);
        
        let right = include_str!("3-corax.txt");
        assert_eq!(left.as_str(), right)
    }

    #[test]
    fn flags() {
        pub const PRG: &[u8] = include_bytes!("../../chip8-test-suite/bin/4-flags.ch8");
        let left = iterate_rom(PRG, 1000);
        
        let right = include_str!("4-flags.txt");
        assert_eq!(left.as_str(), right)
    }
}