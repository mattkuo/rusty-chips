extern crate rand;
extern crate piston_window;

mod emulator;

use std::env;
use std::io::prelude::*;
use std::path::Path;
use piston_window::*;
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

    let mut window: PistonWindow = WindowSettings::new(
        "Rusty Chips",
        [emulator::SCREEN_WIDTH as u32, emulator::SCREEN_HEIGHT as u32]
    ).exit_on_esc(true)
    .build()
    .unwrap();

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {

        if let Some(u) = e.update_args() {
            emulator.update_timer(u.dt);
            emulator.emulate_cycle();
        }

    }

}
