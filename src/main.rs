extern crate rand;
mod emulator;

use std::env;
use std::io::prelude::*;
use std::path::Path;
use emulator::ChipEight;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        let mut stderr = std::io::stderr();
        writeln!(&mut stderr, "usage: {:?} [chip_eight_program]", args[0]).unwrap();
        return;
    }

    let mut emulator = ChipEight::new();
    let prog_path = Path::new(&args[1]);
    emulator.load_memory(&prog_path);

    loop {
        emulator.emulate_cycle();
    }

}
