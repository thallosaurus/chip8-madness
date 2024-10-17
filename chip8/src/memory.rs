use crate::{chip8::ch8_types::{self, MEMORY_SIZE}, display::FONT};
pub struct Memory {
    memory: ch8_types::Memory,
}
impl Default for Memory {
    fn default() -> Self {
        Self { memory: [0; MEMORY_SIZE] }
    }
}
impl Memory {
    pub fn load_at_address(&mut self, address: usize, v: &[u8]) {
        let mut i = 0;
        loop {
            if i > MEMORY_SIZE || i == v.len() {
                break;
            }

            self.memory[address + i] = v[i];

            i += 1;
        }
    }

    pub fn get_u8(&mut self, address: usize) -> &mut u8 {
        &mut self.memory[address]
    }

    pub fn get_instruction(&mut self, address: usize) -> [u8; 2] {
        let (fb, sb) = (self.memory[address], self.memory[address + 1]);
        [fb, sb]
    }
}

#[cfg(test)]
mod tests {
    use crate::display::FONT;
    use super::Memory;

    #[test]
    fn load_at_address() {
        let mut mem = Memory::default();
        mem.load_at_address(0x50, &FONT);
        assert_eq!(mem.memory[0x50], 0xF0);
        assert_eq!(mem.memory[0x51], 0x90);
        assert_eq!(mem.memory[0x52], 0x90);
        assert_eq!(mem.memory[0x53], 0x90);
        assert_eq!(mem.memory[0x54], 0xF0);
    }

    #[test]
    fn get_instruction() {
        let mut mem = Memory::default();

        let data: [u8; 4] = [0x41, 0x42, 0x43, 0x44];
        mem.load_at_address(0x200, &data);
        assert_eq!([0x41, 0x42], mem.get_instruction(0x200))
    }

    #[test]
    fn modify_instruction() {
        let mut mem = Memory::default();

        let data: [u8; 4] = [0x41, 0x42, 0x43, 0x44];
        mem.load_at_address(0x200, &data);
        *mem.get_u8(0x200) = 0x50;

        assert_eq!(0x50, *mem.get_u8(0x200));
        //assert_eq!([0x41, 0x42], mem.get_instruction(0x200))
    }
}