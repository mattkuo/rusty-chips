use std::fs::File;
use std::io::Read;
use std::path::Path;

// use 0x600 for ETI 660 programs
const PROGRAM_MEM_START: usize = 0x200;

pub struct ChipEight {
    opcode: u16,
    memory: [u8; 4096],
    regs: [u8; 16],
    index_reg: u16,
    pc: u16,
    display: [bool; 2048],
    delay_timer: u16,
    sound_timer: u16,
    stack: [u16; 16],
    sp: u16,
    key: [u16; 16]
}


impl ChipEight {

    pub fn new() -> ChipEight {
        ChipEight {
            opcode: 0x0,
            memory: [0x0; 4096],
            regs: [0x0; 16],
            index_reg: 0x0,
            pc: 0x200,
            display: [false; 2048],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0x0; 16],
            sp: 0x0,
            key: [0x0; 16]
        }
    }

    pub fn load_memory(&mut self, path: &Path) {
        let mut file = File::open(path).unwrap();

        let mut buffer = Vec::new();
        let mut bytes_read = file.read_to_end(&mut buffer).unwrap();

        for i in 0..bytes_read {
            self.memory[PROGRAM_MEM_START + i] = buffer[i];
        }
    }

    pub fn emulate_cycle(&mut self) {
        self.opcode = self.fetch();

        match self.opcode & 0xF000 {
            0x6000 => {
                let reg_n = (self.opcode & 0x0F00 >> 2) as usize;
                let value = self.opcode as u8;
                regs[reg_n] = value;
                self.pc += 2;
            },
            0xA000 => {
                index_reg = self.opcode & 0xFFF;
                self.pc += 2;
            }

        }
    }

    fn fetch(&mut self) -> u16 {
        let nibble1 = (self.memory[self.pc as usize] as u16) << 8;
        let nibble2 = self.memory[(self.pc + 1) as usize] as u16;
        nibble1 | nibble2
    }

    fn decode(&self) {

    }

    fn execute(&self) {

    }

}
