extern crate hex;

use hex::FromHex;
use text_io::read;
use std::collections::HashSet;
use super::gbc::Mmu;
use super::gbc::Registers;

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

    pub fn break_at(&self, addr: &u16, mem: &Mmu, regs: &Registers) {
        println!("Hit breakpoint {:04X}\nA: {:02X}, B: {:02X}, C: {:02X}, D: {:02X}, E: {:02X}, F: {:02X}, H: {:02X}, L: {:02X}, SP: {:04X}", 
            addr, regs.a, regs.b, regs.c, regs.d, regs.e, regs.f, regs.h, regs.l, regs.sp);
        println!("flags: Z: {}, H: {}, C: {}, N: {}", 
            regs.zero_flag(), regs.half_carry_flag(), regs.carry_flag(), regs.subtract_flag());
        println!("op {:02X} {:02X} {:02X}", mem.read(regs.pc), mem.read(regs.pc+1), mem.read(regs.pc+2));
        let line: String = read!("{}\n");
        for _part in line.split_whitespace() {

        }
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