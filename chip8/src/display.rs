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

#[derive(Clone, Copy)]
pub struct Chip8Display {
    pub vram: VRAM,
    x: u8,
    y: u8,
}

impl Default for Chip8Display {
    fn default() -> Self {
        Self {
            vram: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for Chip8Display {
    type Item = DisplayStates;

    fn next(&mut self) -> Option<Self::Item> {
        let i = xy_to_i(self.x, self.y);

        if i == self.vram.len() {
            return None
        }
        
        if self.x > (DISPLAY_WIDTH as u8) {
            self.y += 1;
            self.x = 0;
            return Some(DisplayStates::NewLine)
        } else {
            self.x += 1;
        }

        if self.vram[i] {
            Some(DisplayStates::On)
        } else {
            Some(DisplayStates::Off)
        }
    }
}

impl Chip8Display {
    pub fn clear(&mut self) {
        self.vram = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
    }

    pub fn draw(&mut self, mut x: u8, mut y: u8, data: u8) -> u8{
        let mut i = 0;

        let mut changed = 0;

        while i < 8 {
            if x + i > DISPLAY_WIDTH as u8 {
                y += 1;
                x = 0;
            }

            let bit = data & (1 << i) > 0;
            let index = xy_to_i(x, y);


            let state_before = self.vram[index];
            let state_after = state_before ^ bit;
            self.vram[index] ^= bit;

            if state_before && !state_after && changed != 1 {
                changed = 1;
            }

            i += 1;
        }

        changed
    }

    pub fn as_bytes(&self) -> VRAM {
        self.vram
    }

    pub fn as_str(&self) -> [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT + (DISPLAY_HEIGHT)] {
        let mut data: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT + (DISPLAY_HEIGHT)] =
            [PIXEL_OFF; DISPLAY_WIDTH * DISPLAY_HEIGHT + (DISPLAY_HEIGHT)];

        let mut i = 0;
        while i < self.vram.len() {
            if i > 0 && (i % DISPLAY_WIDTH == 0) {
                data[i] = NEWLINE;
            } else {
                data[i] = if self.vram[i] { PIXEL_OFF } else { PIXEL_ON };
            }
            i += 1;
        }

        data
    }
}

fn xy_to_i(x: u8, y: u8) -> usize {
    (y as u16 * DISPLAY_WIDTH as u16 + x as u16).into()
}

#[cfg(test)]
mod tests {
    use crate::display::xy_to_i;

    #[test]
    fn test_xy_to_i() {
        assert_eq!(65, xy_to_i(1, 1))
    }
}
