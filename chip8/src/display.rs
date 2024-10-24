use crate::chip8::ch8_types::{DISPLAY_HEIGHT, DISPLAY_WIDTH, VRAM};

pub enum DisplayStates {
    On,
    Off,
    NewLine,
}

const PIXEL_ON: u8 = 0x58;
const PIXEL_OFF: u8 = 0x20;
pub const NEWLINE: u8 = 0x0a;
pub const CARRIAGE: u8 = 0;

pub const FONT: [u8; 0x50] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// Struct responsible for translation of the memory to string
pub struct DisplayController;

impl DisplayController {
    pub fn clear_vram(&self, obj: &mut VRAM) {
        *obj = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    pub fn draw_onto(&self, obj: &mut VRAM, x: usize, y: usize, data: u8) -> u8 {
        let mut changed = false;
        //let index = global_xy_to_i(x, y);

        //let offset = index as u8 % 8;

        let mut x: usize = x.into();

        let mut pos: u8 = 0;
        while pos != 8 {
            let bitselect: u8 = 1 << 7 - pos;
            let d = (data & bitselect) > 0;

            changed = obj[y][x] & d;

            obj[y][x] ^= d;

            x += 1;
            pos += 1;
        }

        //        if changed { 1 } else { 0 }
        1
    }
}

pub trait Displayable {
    /// Clears the VRAM
    fn clear(&mut self);

    /// Draws onto the memory VRAM Area
    fn draw(&mut self, x: u8, y: u8, data: u8) -> u8;
}

fn global_xy_to_i(x: u8, y: u8) -> usize {
    xy_to_i(x, y, DISPLAY_WIDTH as u16)
}

fn xy_to_i(x: u8, y: u8, width: u16) -> usize {
    (y as u16 * width + x as u16).into()
}

#[cfg(test)]
mod tests {
    use crate::{
        chip8::ch8_types::{DISPLAY_HEIGHT, DISPLAY_WIDTH, VRAM},
        display::{global_xy_to_i, xy_to_i},
    };

    use super::DisplayController;

    #[test]
    fn test_global_xy_to_i() {
        assert_eq!(65, global_xy_to_i(1, 1))
    }

    #[test]
    fn test_xy_to_i() {
        assert_eq!(10, xy_to_i(2, 2, 4))
    }

    #[test]
    fn draw_onto() {
        let mut mem: VRAM = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

        //let rc = &mut mem;

        let controller = DisplayController {};
        controller.draw_onto(&mut mem, 0, 0, 0b11001100);

        assert_eq!(
            mem[0][0..8],
            [false, true, true, false, false, true, true, false]
        )
    }

    /// With this we test if we can successfully write to our VRAM Array
    /// First we draw the specified Bitmask onto the Buffer with the specified offset
    /// Then we check if the data got written correctly
    #[test]
    fn offset_draw_onto() {
        let mut mem: VRAM = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

        let controller = DisplayController {};
        controller.draw_onto(&mut mem, 2, 0, 0b11000000);

        assert_eq!(
            mem[0][0..8],
            [false, false, true, true, false, false, false, false]
        )
    }

    /// Same check as [offset_draw_onto], but with a Y-Offset
    #[test]
    fn offset_draw_onto_y() {
        let mut mem: VRAM = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

        let controller = DisplayController {};
        controller.draw_onto(&mut mem, 2, 1, 0b11000000);

        assert_eq!(
            mem[1][0..8],
            [false, false, true, true, false, false, false, false]
        )
    }
}
