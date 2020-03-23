use std::env;
mod gbc;
mod debugger;

use gbc::Cpu;
use debugger::Debugger;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut gbc = Cpu::new(&args[1]);
        if args.len() > 2 {
            let debugger = Debugger::new(&args[2]);
            gbc.attatch_debugger(debugger);
        }
        gbc.run();
    }

}
