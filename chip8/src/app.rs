use core::cell::RefCell;

use crate::{
    chip8::{
        self,
        ch8_types::{self, MemoryAddress, Registers, Stack, DISPLAY_HEIGHT, DISPLAY_WIDTH, REGISTER_SIZE, STACK_SIZE, VRAM},
        Ops,
    },
    display::{self, DisplayController, FONT},
    memory::Memory,
};

/// Holds the State of the emulator
/// CHIP-8 has the following components:
/// - Memory: CHIP-8 has direct access to up to 4 kilobytes of RAM
/// - Display: 64 x 32 pixels (or 128 x 64 for SUPER-CHIP) monochrome, ie. black or white
/// - A program counter, often called just “PC”, which points at the current instruction in memory
/// - One 16-bit index register called “I” which is used to point at locations in memory
/// - A stack for 16-bit addresses, which is used to call subroutines/functions and return from them
/// - An 8-bit delay timer which is decremented at a rate of 60 Hz (60 times per second) until it reaches 0
/// - An 8-bit sound timer which functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0
/// - 16 8-bit (one byte) general-purpose variable registers numbered 0 through F hexadecimal, ie. 0 through 15 in decimal, called V0 through VF
///     - VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag
#[derive(Debug)]
pub struct AppState {
    pub pc: usize,
    pub sp: usize,
    pub I: u16,
    registers: Registers,
    memory: Memory,
    stack: Stack,
    pub vram: VRAM,
}

impl AppState {
    pub fn new(prog: &[u8]) -> Self {
        // Initialize Memory Layout
        let mut memory = Memory::default();
        memory.load_at_address(0x50, &FONT);
        memory.load_at_address(0x200, prog);

        Self {
            pc: 0x200,
            I: Default::default(),
            sp: Default::default(),
            registers: [0; REGISTER_SIZE],
            memory: memory,
            stack: [0; STACK_SIZE],
            vram: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            //display: Chip8Display::default(),
        }
    }

    fn reset(&mut self) {}

    fn stack_push(&mut self, value: MemoryAddress) {
        self.stack[self.sp] = value;
        self.sp += 1;
    }

    fn stack_pop(&mut self) -> MemoryAddress {
        let value = self.stack[self.sp];
        self.sp -= 1;
        value
    }

    /// Execute next instruction
    /// Returns the Opcode for Debug Purposes
    pub fn step(&mut self) -> Ops {
        let instr: Ops = self.memory.get_instruction(self.pc).into();
        self.exec_op(instr.clone());
        instr
    }

    /// Executes the given Opcode
    fn exec_op(&mut self, i: Ops) {
        //let display = self.getVramController();
        let display = DisplayController {};
        let mem = RefCell::new(&mut self.vram);
        
        match i {
            Ops::CLS => display.clear_vram(*mem.borrow_mut()),
            Ops::RET => {
                let v = self.stack_pop();
                self.pc = v as usize;
                return;
            }
            Ops::JP(addr) => {
                self.pc = addr as usize;
                return;
            }
            Ops::CALL(addr) => {
                let v: u16 = self.pc.try_into().unwrap();
                self.stack_push(v);
                
                self.pc = addr as usize;
                return;
            }
            Ops::DRW(rx, ry, n) => {
                let (x, y) = (self.registers[rx],  self.registers[ry]);
                
                self.registers[0xF] = 0;

                //self.registers[0xF] = display.draw_onto(*mem.borrow_mut(), x, y, n);
                
                let mut i = 0;
                while i < n {
                    // get sprite data from loaded memory
                    let a = self.I + (i as u16);
                    let data = self.memory.get_u8(a.into());

                    // transfer sprite to vram
                    self.registers[0xF] = display.draw_onto(*mem.borrow_mut(), x as usize, (y + i) as usize, *data);
                    i += 1;
                }
            },
            Ops::LD_V(rx, data) => {
                self.registers[rx] = data;
            }
            Ops::ADD_V(rx, data) => {
                self.registers[rx] += data;
            }
            Ops::SET_I(addr) => {
                self.I = addr;
            }
            
            Ops::SYS(addr) => todo!(),
            Ops::SI(rx, data) => {
                if self.registers[rx] == data {
                    self.pc += 2;
                }
            },
            Ops::SIN(rx, data) => {
                if self.registers[rx] != data {
                    self.pc += 2;
                }

            },
            Ops::SVI(rx, ry) => {
                if self.registers[rx] == self.registers[ry] {
                    self.pc += 2;
                }
            },
            Ops::SIV(rx, ry) => {
                self.registers[rx] = self.registers[ry];
            },
            Ops::ORV(rx, ry) => {
                self.registers[rx] |= self.registers[ry];
            },
            Ops::ANDV(rx, ry) => {
                self.registers[rx] &= self.registers[ry];
            },
            Ops::XORV(rx, ry) => {
                self.registers[rx] ^= self.registers[ry];
            },
            Ops::ADDVC(rx, ry) => {
                self.registers[rx] += self.registers[ry];
            },
            Ops::SUBVC(rx, ry) => {
                self.registers[rx] -= self.registers[ry];
            },
            Ops::SHR(rx, ry) => {
                todo!()
            },
            Ops::SUBN(rx, ry) => todo!(),
            Ops::SHL(rx, ry) => todo!(),
            Ops::SNE(rx, ry) => {
                if self.registers[rx] != self.registers[ry] {
                    self.pc += 2;
                }
            },
            Ops::JPV(addr) => {
                self.pc = (self.registers[0] as usize) + (addr as usize);
            },
            Ops::RND(rx, data) => {
                // TODO: Random Number Generator
                todo!()
            },
            Ops::SKP(rx) => {
                // TODO: Keyboard Input
            },
            Ops::SKNP(rx) => todo!(),
            Ops::LDDT(rx) => todo!(),
            Ops::LDK(rx) => todo!(),
            Ops::LDDTE(rx) => todo!(),
            Ops::LDST(rx) => todo!(),
            Ops::ADDI(rx) => {
                self.I = self.I + self.registers[rx] as u16
            },
            Ops::LDF(rx) => {
                self.I = self.registers[rx] as u16
            },
            Ops::LDB(rx) => todo!(),
            Ops::LDI(rx) => {
                let mut i = 0;
                while i < rx {
                    let p = self.memory.get_u8(self.I.into());
                    *p = self.registers[i];
                    i += 1;
                }
            },
            Ops::LDVI(rx) => {
                let mut i = 0;
                while i < rx {
                    let p = self.memory.get_u8(self.I.into());
                    //*p = self.registers[i];
                    self.registers[i] = *p;
                    i += 1;
                }
            },

            // Arbitrary, unhandled Data, possibly unimplemented opcode
            Ops::Data(data) => panic!("Tried executing unhandled opcode, data@{}pc: {:?}", self.pc, data),
        }

        self.pc += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::AppState;

    #[test]
    fn test_app_state() {
        let prg = include_bytes!("../../chip8-roms/roms/IBM Logo.ch8");
        let mut appstate = AppState::new(prg);

        appstate.step();
    }

    #[test]
    fn test_ibm_logo() {
        let prg = include_bytes!("../../chip8-roms/roms/IBM Logo.ch8");
        let mut appstate = AppState::new(prg);

        let mut i = 0;
        while i < 20 {
            appstate.step();
            i += 1;
        }
    }

}
