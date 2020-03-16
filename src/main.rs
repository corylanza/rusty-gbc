use std::env;
mod gbc;

use gbc::Gameboy;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut gbc = Gameboy::new(&args[1]);
    gbc.run();
}
