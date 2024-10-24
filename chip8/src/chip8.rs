use ch8_types::{decode, decode_memory_address};

pub mod ch8_types {
    pub const MEMORY_SIZE: usize = 4096;
    pub const REGISTER_SIZE: usize = 16;

    /// Should be not < 48
    pub const STACK_SIZE: usize = 0xFF;

    pub const DISPLAY_HEIGHT: usize = 32;
    pub const DISPLAY_WIDTH: usize = 64;

    /// The adressed Register Index
    pub type RegisterIndex = usize;

    /// Represents a 4-bit number - 0xF
    pub type Nibble = u8;
    const NIBBLE_MASK: u16 = 0xF;

    /// An 8-bit immediate number - 0xFF
    pub type Byte = u8;
    const BYTE_MASK: u16 = 0xFF;

    /// Represents a 12-bit immediate memory address - 0xFFF
    pub type MemoryAddress = u16;
    const MEMORY_ADDRESS_MASK: u16 = 0xFFF;
    
    pub type VRAM = [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

    pub type Registers = [Byte; REGISTER_SIZE];
    pub type Memory = [Byte; MEMORY_SIZE];
    pub type Stack = [MemoryAddress; STACK_SIZE];

    pub fn decode(i: u16, mask: u16) -> u16 {
        i & mask
    }

    pub fn decode_memory_address(value: u16) -> MemoryAddress {
        decode(value, MEMORY_ADDRESS_MASK)
    }

    pub fn decode_nibble(value: u16) -> Nibble {
        decode(value, NIBBLE_MASK) as u8
    }
}

/// https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
/// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
#[derive(PartialEq, Debug, Clone)]
pub enum Ops {
    /// 00E0 - CLS
    /// Clear the display.
    CLS,

    /// 00EE - RET
    /// Return from a subroutine.
    ///
    /// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.    
    RET,

    /// 1nnn - JP addr
    /// Jump to location nnn.
    ///
    /// The interpreter sets the program counter to nnn.
    JP(ch8_types::MemoryAddress),

    /// 2nnn - CALL addr
    /// Call subroutine at nnn.
    ///
    /// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    CALL(ch8_types::MemoryAddress),

    /// Dxyn - DRW Vx, Vy, nibble
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    ///
    /// The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    DRW(
        ch8_types::RegisterIndex,
        ch8_types::RegisterIndex,
        ch8_types::Nibble,
    ),

    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    ///
    /// The interpreter puts the value kk into register Vx.
    LD_V(ch8_types::RegisterIndex, ch8_types::Byte),

    /// 7xkk - ADD Vx, byte
    /// Set Vx = Vx + kk.
    ///
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    ADD_V(ch8_types::RegisterIndex, ch8_types::Byte),

    /// Annn - LD I, addr
    /// Set I = nnn.
    ///
    /// The value of register I is set to nnn.
    ///
    SET_I(ch8_types::MemoryAddress),

    ///
    /// 0nnn - SYS addr
    /// Jump to a machine code routine at nnn.
    ///
    /// This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
    // SYS,

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    SI(ch8_types::RegisterIndex, ch8_types::Byte),
    
    /// 4xkk - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    /// 
    /// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    SIN(ch8_types::RegisterIndex, ch8_types::Byte),

    /// 5xy0 - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    ///
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    SVI(ch8_types::RegisterIndex, ch8_types::RegisterIndex),

    // 0x8 Math

    /// 8xy0 - LD Vx, Vy
    /// Set Vx = Vy.
    ///
    /// Stores the value of register Vy in register Vx.
    SIV(ch8_types::RegisterIndex, ch8_types::RegisterIndex),
    
    /// 8xy1 - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    ///
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    ORV(ch8_types::RegisterIndex, ch8_types::RegisterIndex),

    /// 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    ///
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
    ANDV(ch8_types::RegisterIndex, ch8_types::RegisterIndex),
    
    /// 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    ///
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    XORV(ch8_types::RegisterIndex, ch8_types::RegisterIndex),
    
    /// 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    ///
    /// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    ADDVC(ch8_types::RegisterIndex, ch8_types::RegisterIndex),

    /// 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    SUBVC(ch8_types::RegisterIndex, ch8_types::RegisterIndex),
    ///
    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    SHR(ch8_types::RegisterIndex, ch8_types::RegisterIndex),

    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    SUBN(ch8_types::RegisterIndex, ch8_types::RegisterIndex),
    
    /// 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    SHL(ch8_types::RegisterIndex, ch8_types::RegisterIndex),

    /// 9xy0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    ///
    /// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    SNE(ch8_types::RegisterIndex, ch8_types::RegisterIndex),

    /// Bnnn - JP V0, addr
    /// Jump to location nnn + V0.
    ///
    /// The program counter is set to nnn plus the value of V0.
    JPV(ch8_types::MemoryAddress),
    
    /// Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.
    ///
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    RND(ch8_types::RegisterIndex, ch8_types::Byte),

    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    SKP(ch8_types::RegisterIndex),
    
    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    SKNP(ch8_types::RegisterIndex),
    
    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    ///
    /// The value of DT is placed into Vx.
    LDDT(ch8_types::RegisterIndex),
    
    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    ///
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    LDK(ch8_types::RegisterIndex),
    ///
    /// Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    ///
    /// DT is set equal to the value of Vx.
    LDDTE(ch8_types::RegisterIndex),
    ///
    /// Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    ///
    /// ST is set equal to the value of Vx.
    LDST(ch8_types::RegisterIndex),
    ///
    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    ///
    /// The values of I and Vx are added, and the results are stored in I.
    ADDI(ch8_types::RegisterIndex),
    ///
    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    ///
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    LDF(ch8_types::RegisterIndex),

    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    ///
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    LDB(ch8_types::RegisterIndex),
    ///
    /// Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location I.
    ///
    /// The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    LDI(ch8_types::RegisterIndex),
    ///
    /// Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    ///
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    LDVI(ch8_types::RegisterIndex),
    
    Data(u16),
}

impl From<[u8; 2]> for Ops {
    fn from(v: [u8; 2]) -> Self {
        let value = (v[0] as u16) << 8 | v[1] as u16;
        match value {
            0x00E0 => Ops::CLS,
            0x00EE => Ops::RET,
            _ => {
                let instr = decode(value, 0xF000) >> 12;
                match instr {
                    0x1 => {
                        let data = decode_memory_address(value);
                        Self::JP(data)
                    }
                    0x2 => {
                        let data = decode_memory_address(value);
                        Self::CALL(data)
                    }
                    0x6 => {
                        let x = (decode(value, 0xF00) as ch8_types::RegisterIndex) >> 8;
                        let kk = (decode(value, 0xFF) as ch8_types::Byte);
                        Self::LD_V(x, kk)
                    }
                    0x7 => {
                        let x = (decode(value, 0xF00) as ch8_types::RegisterIndex) >> 8;
                        let kk = (decode(value, 0xFF) as ch8_types::Byte);
                        Self::ADD_V(x, kk)
                    }
                    0xA => {
                        let data = decode_memory_address(value);
                        Self::SET_I(data)
                    }
                    0xD => {
                        let x = (decode(value, 0xF00) as ch8_types::RegisterIndex) >> 8;
                        let y = (decode(value, 0xF0) as ch8_types::RegisterIndex) >> 4;
                        let n = decode(value, 0xF) as ch8_types::Nibble;
                        Self::DRW(x, y, n)
                    }
                    0x3 => {
                        let x = (decode(value, 0xF00) as ch8_types::RegisterIndex) >> 8;
                        let kk = decode(value, 0xFF) as ch8_types::Byte;
                        Self::SI(x, kk)
                    }
                    0x4 => {
                        let x = (decode(value, 0xF00) as ch8_types::RegisterIndex) >> 8;
                        let kk = decode(value, 0xFF) as ch8_types::Byte;
                        Self::SIN(x, kk)
                    }
                    0x5 => {
                        let x = (decode(value, 0xF00) as ch8_types::RegisterIndex) >> 8;
                        let y = (decode(value, 0xF0) as ch8_types::RegisterIndex) >> 4;
                        Self::SVI(x, y)
                    }
                    0x8 => {
                        let x = (decode(value, 0xF00) as ch8_types::RegisterIndex) >> 8;
                        let y = (decode(value, 0xF0) as ch8_types::RegisterIndex) >> 4;
                        
                        return match decode(value, 0xF) as ch8_types::Nibble {
                            0 => {
                                Self::SIV(x, y)
                            }
                            1 => {
                                Self::ORV(x, y)
                            }
                            2 => {
                                Self::ANDV(x, y)
                            }
                            3 => {
                                Self::XORV(x, y)
                            }
                            4 => {
                                Self::ADDVC(x, y)
                            }
                            5 => {
                                Self::SUBVC(x, y)
                            }
                            6 => {
                                Self::SHR(x, y)
                            }
                            7 => {
                                Self::SUBN(x, y)
                            }
                            0xE => {
                                Self::SHL(x, y)
                            }
                            _ => {
                                panic!("Opcode not defined: {}", instr)
                            }
                        }
                    }
                    0x9 => {
                        let x = (decode(value, 0xF00) as ch8_types::RegisterIndex) >> 8;
                        let y = (decode(value, 0xF0) as ch8_types::RegisterIndex) >> 4;
                        Self::SNE(x, y)
                    }
                    0xB => {
                        let data = decode_memory_address(value);
                        Self::JPV(data)
                    }
                    0xC => {
                        let x = (decode(value, 0xF00) as ch8_types::RegisterIndex) >> 8;
                        let kk = decode(value, 0xFF) as ch8_types::Byte;
                        Self::RND(x, kk)
                    }
                    0xE => {
                        let x = (decode(value, 0xF00) as ch8_types::RegisterIndex) >> 8;
                        match decode(value, 0xFF) as ch8_types::Byte {
                            0x9E => {
                                Self::SKP(x)
                            }
                            0xA1 => {
                                Self::SKNP(x)
                            }
                            _ => {
                                panic!("Unknown Opcode")
                            }
                        }
                    }
                    0xF => {
                        let x = (decode(value, 0xF00) as ch8_types::RegisterIndex) >> 8;
                        match decode(value, 0xFF) as ch8_types::Byte {
                            0x07 => {
                                Self::LDDT(x)
                            }
                            0x0A => {
                                Self::LDK(x)
                            }
                            0x15 => {
                                Self::LDDTE(x)
                            }
                            0x18 => {
                                Self::LDST(x)
                            }
                            0x1E => {
                                Self::ADDI(x)
                            }
                            0x29 => {
                                Self::LDF(x)
                            }
                            0x33 => {
                                Self::LDB(x)
                            }
                            0x55 => {
                                Self::LDI(x)
                            }
                            0x65 => {
                                Self::LDVI(x)
                            }
                            _ => {
                                panic!("Opcode not found: {}", instr)
                            }
                        }
                    }
                    _ => todo!("Opcode not defined: {}", instr),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let data = 0x3366_u16;
        assert_eq!(0x66, decode(data, 0xFF))
    }

    #[test]
    fn clear_screen() {
        let opcode = [0x00, 0xE0];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::CLS, instr);
    }

    #[test]
    fn jump() {
        let opcode = [0x1A, 0xAA];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::JP(0xAAA_u16), instr);
    }

    #[test]
    fn call() {
        let opcode = [0x2A, 0xAA];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::CALL(0xAAA_u16), instr);
    }

    #[test]
    fn set_register_vx() {
        let opcode = [0x60, 0xFF];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::LD_V(0, 0xff), instr);
    }

    #[test]
    fn add_value_to_register() {
        let opcode = [0x70, 0xFF];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::ADD_V(0, 0xff), instr);
    }

    #[test]
    fn set_index_register_i() {
        let opcode = [0xAB, 0xBB];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::SET_I(0xBBB), instr);
    }

    #[test]
    fn display_draw() {
        let opcode = [0xD1, 0x24];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::DRW(0x1, 0x2, 0x4), instr);
    }

    #[test]
    fn skip_next_instruction_if_vx_kk() {
        let opcode = [0x35, 0x42];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::SI(0x5, 0x42), instr);
    }

    #[test]
    fn skip_next_instruction_if_vx_not_kk() {
        let opcode = [0x45, 0x42];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::SIN(0x5, 0x42), instr);
    }

    #[test]
    fn skip_next_instruction_if_vx_vy() {
        let opcode = [0x55, 0x40];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::SVI(0x5, 0x4), instr);
    }

    #[test]
    fn store_y_in_x() {
        let opcode = [0x81, 0x20];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::SIV(0x1, 0x2), instr);
    }

    #[test]
    fn or_vx_vy() {
        let opcode = [0x81, 0x21];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::ORV(0x1, 0x2), instr);
    }

    #[test]
    fn and_vx_vy() {
        let opcode = [0x81, 0x22];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::ANDV(0x1, 0x2), instr);
    }

    #[test]
    fn xor_vx_vy() {
        let opcode = [0x81, 0x23];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::XORV(0x1, 0x2), instr);
    }

    #[test]
    fn add_vx_vy() {
        let opcode = [0x81, 0x24];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::ADDVC(0x1, 0x2), instr);
    }

    #[test]
    fn sub_vx_vy() {
        let opcode = [0x81, 0x25];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::SUBVC(0x1, 0x2), instr);
    }

    #[test]
    fn shr_vx_vy() {
        let opcode = [0x81, 0x26];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::SHR(0x1, 0x2), instr);
    }

    #[test]
    fn subn_vx_vy() {
        let opcode = [0x81, 0x27];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::SUBN(0x1, 0x2), instr);
    }

    #[test]
    fn shl_vx_vy() {
        let opcode = [0x81, 0x2E];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::SHL(0x1, 0x2), instr);
    }

    #[test]
    fn sne_vx_vy() {
        let opcode = [0x91, 0x20];
        let instr: Ops = opcode.into();
        assert_eq!(Ops::SNE(0x1, 0x2), instr);
    }

    #[test]
    fn jpv() {
        let opcode = [0xBA, 0xAA];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::JPV(0xAAA_u16), instr);
    }

    #[test]
    fn rnd() {
        let opcode = [0xC0, 0xFF];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::RND(0, 0xff), instr);
    }

    #[test]
    fn skp() {
        let opcode = [0xE1, 0x9E];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::SKP(1), instr);
    }

    #[test]
    fn sknp() {
        let opcode = [0xE1, 0xA1];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::SKNP(1), instr);
    }

    #[test]
    fn lddt() {
        let opcode = [0xF1, 0x07];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::LDDT(1), instr);
    }

    #[test]
    fn ldk() {
        let opcode = [0xF1, 0x0A];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::LDK(1), instr);
    }

    #[test]
    fn lddte() {
        let opcode = [0xF1, 0x15];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::LDDTE(1), instr);
    }

    #[test]
    fn ldst() {
        let opcode = [0xF1, 0x18];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::LDST(1), instr);
    }

    #[test]
    fn addi() {
        let opcode = [0xF1, 0x1E];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::ADDI(1), instr);
    }

    #[test]
    fn ldf() {
        let opcode = [0xF1, 0x29];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::LDF(1), instr);
    }

    #[test]
    fn ldb() {
        let opcode = [0xF1, 0x33];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::LDB(1), instr);
    }

    #[test]
    fn ldi() {
        let opcode = [0xF1, 0x55];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::LDI(1), instr);
    }

    #[test]
    fn ldvi() {
        let opcode = [0xF1, 0x65];
        let instr: Ops = opcode.into();

        assert_eq!(Ops::LDVI(1), instr);
    }
}
