pub const SCREEN_W: usize = 64;
pub const SCREEN_H: usize = 32;
const MEM_SIZE: usize = 4096;
const REG_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

pub struct Emu {
    pc: u16,
    mem: [u8; MEM_SIZE],
    sc: [bool; SCREEN_W * SCREEN_H],
    v_reg: [u8; REG_SIZE],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    d_timer: u8,
    s_timer: u8,
}

impl Emu {
    pub fn new() -> Self {
        Self {
            pc: 0x200,
            mem: [0; MEM_SIZE],
            sc: [false; SCREEN_W * SCREEN_H],
            v_reg: [0; REG_SIZE],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            d_timer: 0,
            s_timer: 0,
        }
    }
}
