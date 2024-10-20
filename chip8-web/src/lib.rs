mod utils;

use core::str;

use chip8::{app::AppState, chip8::ch8_types::DISPLAY_WIDTH, display::DisplayController};
use wasm_bindgen::prelude::*;

pub const IBM_LOGO: &[u8] = include_bytes!("../../chip8-roms/roms/IBM Logo.ch8");

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, chip8-web!");
}

#[wasm_bindgen]
pub struct Chip8JsController {
    state: AppState
}

#[wasm_bindgen]
impl Chip8JsController {

    #[wasm_bindgen(constructor)]
    pub fn new(prog: &[u8]) -> Self {
        Self {
            state: AppState::new(prog)
        }
    }

    #[wasm_bindgen]
    pub fn step(&mut self) {
        self.state.step();
    }
    
    #[wasm_bindgen]
    pub fn get_output(&self) -> Vec<String>{
        let data: Vec<String> = self.state.vram.iter().map(|f|{
            row_to_string(f)
        }).collect();
    
        data
    }
}

pub fn row_to_string(o: &[bool; DISPLAY_WIDTH]) -> String {
    let mut s = String::new();

    let mut i = 7;
    while i < o.len() {
        s.push(if o[i] {
            'X'
        } else {
            ' '
        });
        i += 1;
    }

    s.push('\n');

    s
}

#[wasm_bindgen]
pub fn init_rt() -> JsValue {
    utils::set_panic_hook();
    Chip8JsController::new(IBM_LOGO).into()
}

#[cfg(test)]
mod tests {
    use chip8::{chip8::ch8_types::{DISPLAY_HEIGHT, DISPLAY_WIDTH}, display::DisplayController};

    use crate::row_to_string;


    #[test]
    fn test_row_to_string() {
        let mut mem = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

        let controller = DisplayController {};

        controller.draw_onto(&mut mem, 0, 0, 0b11001100);
        assert_eq!(String::from("XX  XX  "), row_to_string(&mem[0]))
    }
}