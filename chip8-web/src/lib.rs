mod utils;

use core::str;

use chip8::app::AppState;
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
    pub fn get_output(self) -> Vec<String>{

        //expect("No runtime loaded");
        let data: Vec<String> = self.state.display_iter().map(|f| {
            match f {
                chip8::display::DisplayStates::On => {
                    String::from("X")
                },
                chip8::display::DisplayStates::Off => {
                    String::from("_")
                },
                chip8::display::DisplayStates::NewLine => {
                    String::from("\r")
                },
            }
        }).collect();
    
        data
    }
}

#[wasm_bindgen]
pub fn init_rt() -> JsValue {
    utils::set_panic_hook();
    Chip8JsController::new(IBM_LOGO).into()
}