use std::env;
mod gbc;

use gbc::Cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut gbc = Cpu::new(&args[1]);
    gbc.run();
}
