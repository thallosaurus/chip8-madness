mod dom;
mod utils;

use core::str;
use std::{cell::RefCell, rc::Rc};

use chip8::{
    app::AppState,
    chip8::{
        ch8_types::{DISPLAY_WIDTH, STACK_SIZE},
        Ops,
    },
};
use dom::{request_animation_frame, set_timeout, update_canvas, window};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(debug_assertions)]
pub const PRG: &[u8] = include_bytes!("../../chip8-test-suite/bin/3-corax+.ch8");

#[cfg(not(debug_assertions))]
pub const PRG: &[u8] = include_bytes!("../../chip8-roms/roms/Pong (alt).ch8");

#[wasm_bindgen(start)]
fn run() {
    #[cfg(debug_assertions)]
    utils::set_panic_hook();
    let mut rt = Rc::new(RefCell::new(AppState::new(PRG)));

    // js quirks
    {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let rt = rt.clone();

        *g.borrow_mut() = Some(Closure::new(move || {
            let mut rt = rt.borrow_mut();
            let inst = rt.step();

            if let Err(data) = inst {
                console::error_1(&JsValue::from_str(&format!("unknown opcode: {}", data)));
                let o: Ops = data.into();
                debug_output(&rt, &o);
                return;
            }

            
            #[cfg(debug_assertions)]
            debug_output(&rt, &inst.ok().unwrap());
            
            set_timeout(f.borrow().as_ref().unwrap(), 17);
        }));
        
        set_timeout(g.borrow().as_ref().unwrap(), 17);
        
        //tick.forget();
    }
    
    {
        // animation frame
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        
        let rt = rt.clone();

        *g.borrow_mut() = Some(Closure::new(move || {
            let mut rt = rt.borrow_mut();
            rt.dec_timers();
            update_canvas(&rt.vram);
            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}


fn debug_output(rt: &AppState, inst: &Ops) {
    let dbg_str = format!(
        "[DEBUG] OP: {:?}, PC: {}, I: {}, SP: {}\n\n{:?}",
        inst,
        rt.pc,
        rt.I,
        rt.sp,
            rt.get_stack()
    );
    console::log_1(&JsValue::from_str(&dbg_str));
}

#[cfg(test)]
mod tests {
    use chip8::{
        app::AppState,
        chip8::ch8_types::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
        display::DisplayController,
    };

    pub const IBM_LOGO: &[u8] = include_bytes!("../../chip8-roms/roms/IBM Logo.ch8");

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

/*    #[test]
    fn test_row_to_stringx() {
        let mut mem = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

        let controller = DisplayController {};

        controller.draw_onto(&mut mem, 0, 0, 0b11001100);
        assert_eq!(String::from("XX  XX  "), row_to_string(&mem[0]))
    }*/
}
