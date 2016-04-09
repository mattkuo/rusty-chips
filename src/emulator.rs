use std::fs::File;
use std::io::Read;
use std::path::Path;

// use 0x600 for ETI 660 programs
const PROGRAM_MEM_START: usize = 0x200;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const FONT_MAP: [u8; 5 * 16] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct ChipEight {
    opcode: u16,
    memory: [u8; 4096],
    regs: [u8; 16],
    index_reg: u16,
    pc: u16,
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    delay_timer: u16,
    sound_timer: u16,
    stack: [u16; 16],
    sp: usize,
    keypad: [u16; 16],
    draw_flag: bool
}


impl ChipEight {

    pub fn new() -> ChipEight {
        let mut chip_eight = ChipEight {
            opcode: 0x0,
            memory: [0x0; 4096],
            regs: [0x0; 16],
            index_reg: 0x0,
            pc: 0x200,
            display: [false; 2048],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0x0; 16],
            sp: 0,
            keypad: [0x0; 16],
            draw_flag: false
        };

        for index in 0..FONT_MAP.len() {
            chip_eight.memory[index] = FONT_MAP[index];
        }

        return chip_eight;
    }

    pub fn load_memory(&mut self, path: &Path) {
        let mut file = File::open(path).unwrap();

        let mut buffer = Vec::new();
        let bytes_read = file.read_to_end(&mut buffer).unwrap();

        for i in 0..bytes_read {
            self.memory[PROGRAM_MEM_START + i] = buffer[i];
        }
    }

    pub fn emulate_cycle(&mut self) {
        self.opcode = self.fetch();

        match self.opcode & 0xF000 {
            // 2NNN	Calls subroutine at NNN
            0x2000 => {
                self.stack[self.sp] = self.pc + 2;
                self.sp += 1;
                self.pc = self.opcode & 0xFFF;
            }
            // 6XNN Sets VX to NN.
            0x6000 => {
                let reg_n = (self.opcode & 0xF00 >> 8) as usize;
                let value = self.opcode as u8;
                self.regs[reg_n] = value;
                self.pc += 2;
            },
            // ANNN	Sets index_reg to the address NNN.
            0xA000 => {
                self.index_reg = self.opcode & 0xFFF;
                self.pc += 2;
            },
            // DXYN Display drawing code
            0xD000 => {
                let start_x = (self.opcode & 0xF00 >> 8) as usize;
                let start_y = (self.opcode & 0xF0 >> 4) as usize;
                let rows = (self.opcode & 0xF) as usize;

                // Set to true on collision
                self.regs[0xF] = 0;

                for row in 0..rows {
                    let row_pixels = self.memory[self.index_reg as usize + row];
                    for col in 0..8 {
                        if 0x80 >> col & row_pixels == 0 { continue; }

                        let current_coord = (SCREEN_WIDTH * (start_y + rows)) + start_x + col;
                        if self.display[current_coord] {
                            self.regs[0xF] = 1;
                        }

                        self.display[current_coord] ^= true;
                    }
                }

                self.draw_flag = true;
                self.pc += 2;
            },
            0xF000 => {
                let x_value = self.regs[(self.opcode & 0xF00 >> 8) as usize];

                match self.opcode & 0xFF {
                    // FX33
                    0x33 => {
                        let mut x_value = x_value;
                        for offset in (0..3).rev() {
                            self.memory[self.index_reg as usize + offset] = x_value % 10;
                            x_value /= 10;
                        }
                        self.pc += 2;
                    },
                    // FX65
                    0x65 => {
                        let mut current_address = self.index_reg as usize;
                        for reg in 0..x_value + 1 {
                            self.regs[reg as usize] = self.memory[current_address];
                            current_address += 1;
                        }
                        self.pc += 2;
                    },
                    instruction => println!("Unknown instructions: {:x}", instruction)
                }
            },
            instruction => println!("Unknown instructions: {:x}", instruction)
        }
    }

    fn fetch(&mut self) -> u16 {
        let nibble1 = (self.memory[self.pc as usize] as u16) << 8;
        let nibble2 = self.memory[(self.pc + 1) as usize] as u16;
        nibble1 | nibble2
    }

}
