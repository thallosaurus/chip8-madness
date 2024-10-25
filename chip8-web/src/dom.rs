use chip8::chip8::ch8_types::{DISPLAY_HEIGHT, DISPLAY_WIDTH, VRAM};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> Document {
    window().document().expect("no global document exists")
}

pub fn write_to_output_window(data: String) {
    let elem = document()
        .query_selector("pre#output")
        .expect("no output element");

    elem.unwrap().set_inner_html(data.as_str());
}

pub fn set_timeout(f: &Closure<dyn FnMut()>, time: i32) {
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), time)
        .expect("should register `setTimeout` OK");
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register 'requestAnimationFrame' OK")
}

pub fn update_canvas(data: &VRAM) {
    let canvas = document().get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let mut y = 0;
    while y < DISPLAY_HEIGHT {
        let mut x = 0;
        while x < DISPLAY_WIDTH {
            ctx.set_fill_style_str(

                if data[y][x] {
                    "white"
                } else {
                    "black"
                }
            );
            ctx.fill_rect((x * 10) as f64, (y * 10) as f64, 10_f64, 10_f64);
            x += 1;
        }
        y += 1;
    }
}
