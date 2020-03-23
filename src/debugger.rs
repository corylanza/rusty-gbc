extern crate hex;

use hex::FromHex;
use text_io::read;
use std::collections::HashSet;
use super::gbc::memory::Memory;
use super::gbc::memory::Registers;

pub struct Debugger {
    pub breakpoints: HashSet<u16>
}

impl Debugger {
    pub fn new(breakpoint_list: &str) -> Self {
        let mut bps = HashSet::<u16>::new();

        for bp in breakpoint_list.split_whitespace() {
            bps.insert(u16_fromhex(&bp).unwrap());
        }

        Debugger {
            breakpoints: bps
        }
    }

    pub fn break_at(&self, addr: &u16, mem: &Memory, regs: &Registers) {
        println!("Hit breakpoint {:04X}\n A: {:02X}, B: {:02X}, C: {:02X}, D: {:02X}, E: {:02X}, SP: {:04X},", 
            addr, regs.a, regs.b, regs.c, regs.d, regs.e, regs.sp);
        let line: String = read!("{}\n");
    }
}

fn u16_fromhex(hex: &str) -> Option<u16> {
    match <[u8; 2]>::from_hex(hex) {
        Ok(bytes) => Some(u16::from_be_bytes([bytes[0], bytes[1]])),
        Err(e) => {
            println!("Debugger failed to attatched to breakpoint: {}", e);
            None
        }
    }
}