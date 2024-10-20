use core::str;
use std::{
    cell::RefCell,
    thread,
    time::{Duration, SystemTime},
};

use chip8::chip8::ch8_types::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub const IBM_LOGO: &[u8] = include_bytes!("../../chip8-roms/roms/IBM Logo.ch8");

fn main() {
    let mut app = chip8::app::AppState::new(IBM_LOGO);

    loop {        
        app.step();

        let mut y = 0;

        let mut data = String::new();

        while y < DISPLAY_HEIGHT {
            
            let mut x = 0;
            while x < DISPLAY_HEIGHT {
                
                let mut i = 0;
                let bit = app.vram[y][x];

                data.push(if bit {
                    'X'
                } else {
                    '.'
                });
                x += 1;
            }

            data.push('\n');

            y += 1;
        }
    
        println!("{}", data);
        
        std::thread::sleep(Duration::new(1, 0));
    }

    //ufmt::uwriteln!(&mut serial, "Hello from ATmega!\r").unwrap();
}

/*fn int_to_string(n: ) -> String {
    let mut s = String::new();
    
    let mut i = 7;
    while i > 0 {
        if n & 1 << i > 0 {
            s.push('X');
        } else {
            s.push(' ');
        }
        i -= 1;
    }

    s
}*/