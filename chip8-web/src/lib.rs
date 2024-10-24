mod dom;
mod utils;

use core::str;

use chip8::{app::AppState, chip8::ch8_types::DISPLAY_WIDTH, display::DisplayController};
use dom::{update_canvas, window};
use wasm_bindgen::prelude::*;
use web_sys::console;

pub const IBM_LOGO: &[u8] = include_bytes!("../../chip8-roms/roms/IBM Logo.ch8");

#[wasm_bindgen(start)]
fn run() {
    let mut rt = AppState::new(IBM_LOGO);

    let tick = Closure::<dyn FnMut()>::new(move || {
        let inst = rt.step();
        update_canvas(&rt.vram);

        let dbg_str = format!("[DEBUG] OP: {:?}, PC: {}, I: {}, SP: {}", inst, rt.pc, rt.I, rt.sp);
        
        #[cfg(debug_assertions)]
        console::log_1(&JsValue::from_str(&dbg_str));
    });

    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(tick.as_ref().unchecked_ref(), 100)
        .expect("error");

    tick.forget();
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

#[cfg(test)]
mod tests {
    use chip8::{
        chip8::ch8_types::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
        display::DisplayController,
    };

    use crate::row_to_string;

    #[test]
    fn test_row_to_string() {
        let mut mem = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

        let controller = DisplayController {};

        controller.draw_onto(&mut mem, 0, 0, 0b11001100);
        assert_eq!(String::from("XX  XX  "), row_to_string(&mem[0]))
    }
}
