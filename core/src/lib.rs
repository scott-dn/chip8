pub const SCREEN_W: usize = 64;
pub const SCREEN_H: usize = 32;
const MEM_SIZE: usize = 4096;
const REG_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
const KEY_SIZE: usize = 16;

const STR_ADDR: u16 = 0x200;

const RESERVED_SPRITE_SIZE: usize = 80;
const RESERVED_SPRITES: [u8; RESERVED_SPRITE_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0,
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

pub struct Emu {
    pc: u16,
    mem: [u8; MEM_SIZE],
    sc: [bool; SCREEN_W * SCREEN_H],
    reg: [u8; REG_SIZE],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; KEY_SIZE],
    d_timer: u8,
    s_timer: u8,
}

impl Emu {
    pub fn default() -> Self {
        let mut mem = [0; MEM_SIZE];
        mem[..RESERVED_SPRITE_SIZE].copy_from_slice(&RESERVED_SPRITES);
        Self {
            pc: STR_ADDR,
            mem,
            sc: [false; SCREEN_W * SCREEN_H],
            reg: [0; REG_SIZE],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; KEY_SIZE],
            d_timer: 0,
            s_timer: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = STR_ADDR;
        self.mem = [0; MEM_SIZE];
        self.mem[..RESERVED_SPRITE_SIZE].copy_from_slice(&RESERVED_SPRITES);
        self.sc = [false; SCREEN_W * SCREEN_H];
        self.reg = [0; REG_SIZE];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; KEY_SIZE];
        self.d_timer = 0;
        self.s_timer = 0;
    }

    pub fn load(&mut self, data: &[u8]) {
        self.mem[STR_ADDR as usize..STR_ADDR as usize + data.len()].copy_from_slice(data);
    }

    pub fn display(&self) -> &[bool] {
        &self.sc
    }

    pub fn key_press(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn tick_timers(&mut self) {
        if self.d_timer > 0 {
            self.d_timer -= 1;
        }

        if self.s_timer > 0 {
            if self.s_timer == 1 {
                println!("S_TIMER");
            }
            self.s_timer -= 1;
        }
    }

    pub fn exec(&mut self) {
        let op = (self.mem[self.pc as usize] as u16) << 8 | self.mem[(self.pc + 1) as usize] as u16;
        self.pc += 2;

        match (
            ((op & 0xF000) >> 12) as u8,
            ((op & 0xF00) >> 8) as u8,
            ((op & 0xF0) >> 4) as u8,
            (op & 0xF) as u8,
        ) {
            // Clears the screen.
            (0, 0, 0xE, 0) => {
                self.sc = [false; SCREEN_W * SCREEN_H];
            }

            // Returns from a subroutine.
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }

            // Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN.
            // Not necessary for most ROMs.
            (0, ..) => {}

            // Jumps to address NNN.
            (1, ..) => {
                self.pc = op & 0xFFF;
            }

            // Calls subroutine at NNN.
            (2, ..) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = op & 0xFFF;
            }

            // Skips the next instruction if VX equals NN
            // (usually the next instruction is a jump to skip a code block)
            (3, x, ..) => {
                if self.reg[x as usize] == (op & 0xFF) as u8 {
                    self.pc += 2;
                }
            }

            // Skips the next instruction if VX does not equal NN
            // (usually the next instruction is a jump to skip a code block)
            (4, x, ..) => {
                if self.reg[x as usize] != (op & 0xFF) as u8 {
                    self.pc += 2;
                }
            }

            // Skips the next instruction if VX equals VY
            // (usually the next instruction is a jump to skip a code block).
            (5, x, y, 0) => {
                if self.reg[x as usize] == self.reg[y as usize] {
                    self.pc += 2;
                }
            }

            // Sets VX to NN.
            (6, x, ..) => {
                self.reg[x as usize] = (op & 0xFF) as u8;
            }

            // Adds NN to VX (carry flag is not changed).
            (7, x, ..) => {
                self.reg[x as usize] = self.reg[x as usize].wrapping_add((op & 0xFF) as u8);
            }

            // Sets VX to the value of VY.
            (8, x, y, 0) => {
                self.reg[x as usize] = self.reg[y as usize];
            }

            // Sets VX to VX or VY.
            // (bitwise OR operation)
            (8, x, y, 1) => {
                self.reg[x as usize] |= self.reg[y as usize];
            }

            // Sets VX to VX and VY.
            // (bitwise AND operation)
            (8, x, y, 2) => {
                self.reg[x as usize] &= self.reg[y as usize];
            }

            // Sets VX to VX xor VY.
            // (bitwise XOR operation)
            (8, x, y, 3) => {
                self.reg[x as usize] ^= self.reg[y as usize];
            }

            // Adds VY to VX.
            // VF is set to 1 when there's a carry,
            // and to 0 when there is not.
            (8, x, y, 4) => {
                let (res, carry) = self.reg[x as usize].overflowing_add(self.reg[y as usize]);
                self.reg[x as usize] = res;
                self.reg[0xF] = carry.into();
            }

            // VY is subtracted from VX.
            // VF is set to 0 when there's a borrow,
            // and 1 when there is not.
            (8, x, y, 5) => {
                let (res, borrow) = self.reg[x as usize].overflowing_sub(self.reg[y as usize]);
                self.reg[x as usize] = res;
                self.reg[0xF] = borrow.into();
            }

            // Stores the least significant bit of VX in VF
            // and then shifts VX to the right by 1.
            (8, x, _y, 6) => {
                self.reg[0xF] = self.reg[x as usize] & 1;
                self.reg[x as usize] >>= 1;
            }

            // Sets VX to VY minus VX.
            // VF is set to 0 when there's a borrow,
            // and 1 when there is not.
            (8, x, y, 7) => {
                let (res, borrow) = self.reg[y as usize].overflowing_sub(self.reg[x as usize]);
                self.reg[x as usize] = res;
                self.reg[0xF] = borrow.into();
            }

            // Stores the most significant bit of VX in VF
            // and then shifts VX to the left by 1.
            (8, x, _y, 0xE) => {
                self.reg[0xF] = self.reg[x as usize] >> 7 & 1;
                self.reg[x as usize] <<= 1;
            }

            // Skips the next instruction if VX does not equal VY.
            // (Usually the next instruction is a jump to skip a code block);
            (9, x, y, 0) => {
                if self.reg[x as usize] != self.reg[y as usize] {
                    self.pc += 2;
                }
            }

            // Sets I to the address NNN.
            (0xA, ..) => {
                self.i_reg = op & 0xFFF;
            }

            // Jumps to the address NNN plus V0.
            (0xB, ..) => {
                self.pc = (op & 0xFFF) + self.reg[0] as u16;
            }

            // Sets VX to the result of a bitwise and operation on a random number
            // (Typically: 0 to 255) and NN.
            (0xC, x, ..) => {
                self.reg[x as usize] = (op & 0xFF) as u8 & rand::random::<u8>();
            }

            // Draws a sprite at coordinate (VX, VY)
            // that has a width of 8 pixels and a height of N pixels.
            // Each row of 8 pixels is read as bit-coded starting from memory location I;
            // I value does not change after the execution of this instruction.
            // As described above,
            // VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn,
            // and to 0 if that does not happen.
            // NOTE: sprite pixels that are set flip the color of the corresponding screen pixel,
            // while unset sprite pixels do nothing
            (0xD, x, y, n) => {
                let (x, y, mut flipped) = (self.reg[x as usize], self.reg[y as usize], false);
                for i in 0..n {
                    let colors = self.mem[(self.i_reg + i as u16) as usize];
                    for j in 0..8 {
                        // check if the sprite is set; otherwise, do nothing.
                        if colors & (0b1000_0000 >> j) > 0 {
                            // screen overflow
                            let (x, y) = ((x + j) as usize % SCREEN_W, (y + i) as usize % SCREEN_H);

                            // 2D -> 1D
                            let idx = x + y * SCREEN_W;

                            flipped |= self.sc[idx];
                            self.sc[idx] ^= true;
                        }
                    }
                }
                self.reg[0xF] = flipped.into();
            }

            // Skips the next instruction if the key stored in VX is pressed
            // (usually the next instruction is a jump to skip a code block).
            (0xE, x, 9, 0xE) => {
                if self.keys[self.reg[x as usize] as usize] {
                    self.pc += 2;
                }
            }

            // Skips the next instruction if the key stored in VX is not pressed
            // (usually the next instruction is a jump to skip a code block).
            (0xE, x, 0xA, 1) => {
                if !self.keys[self.reg[x as usize] as usize] {
                    self.pc += 2;
                }
            }

            // Sets VX to the value of the delay timer.
            (0xF, x, 0, 7) => {
                self.reg[x as usize] = self.d_timer;
            }

            // A key press is awaited, and then stored in VX
            // (blocking operation, all instruction halted until next key event).
            (0xF, x, 0, 0xA) => {
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.reg[x as usize] = i as u8;
                        return;
                    }
                }
                self.pc -= 2;
            }

            // Sets the delay timer to VX.
            (0xF, x, 1, 5) => {
                self.d_timer = self.reg[x as usize];
            }

            // Sets the sound timer to VX.
            (0xF, x, 1, 8) => {
                self.s_timer = self.reg[x as usize];
            }

            // Adds VX to I. VF is not affected.
            (0xF, x, 1, 0xE) => {
                self.i_reg = self.i_reg.wrapping_add(self.reg[x as usize] as u16);
            }

            // Sets I to the location of the sprite for the character in VX.
            // Characters 0-F (in hexadecimal) are represented by a 4x5 font.
            (0xF, x, 2, 9) => {
                self.i_reg = self.reg[x as usize] as u16 * 5;
            }

            // Stores the binary-coded decimal representation of VX,
            // with the hundreds digit in memory at location in I,
            // the tens digit at location I+1, and the ones digit at location I+2.
            (0xF, x, 3, 3) => {
                self.mem[self.i_reg as usize] = self.reg[x as usize] / 100;
                self.mem[(self.i_reg + 1) as usize] = self.reg[x as usize] % 100 / 10;
                self.mem[(self.i_reg + 2) as usize] = self.reg[x as usize] % 10;
            }

            // Stores from V0 to VX (including VX) in memory, starting at address I.
            // The offset from I is increased by 1 for each value written, but I itself is left unmodified.
            (0xF, x, 5, 5) => {
                for i in 0..=x as u16 {
                    self.mem[(self.i_reg + i) as usize] = self.reg[i as usize];
                }
            }

            // Fills from V0 to VX (including VX) with values from memory, starting at address I.
            // The offset from I is increased by 1 for each value read, but I itself is left unmodified.
            (0xF, x, 6, 5) => {
                for i in 0..=x as u16 {
                    self.reg[i as usize] = self.mem[(self.i_reg + i) as usize];
                }
            }

            _ => unimplemented!("Unsupported opcode: {:#04x}", op),
        }
    }
}
