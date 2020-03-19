mod rom;
mod ram;
mod registers;

pub use registers::Registers;
use rom::Rom;
use ram::Ram;
use std::str;

const ROM_START: u16 = 0;
const ROM_END: u16 = 0x7FFF;
const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const ERAM_START: u16 = 0xA000;
const ERAM_END: u16 = 0xBFFF;
const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xDFFF;
const ECHO_START: u16 = 0xC000;
const ECHO_END: u16 = 0xFDFF;
const OAM_START: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;
const IO_START: u16 = 0xFF00;
const IO_END: u16 = 0xFF7F;
const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;
const INTERUPTS_ENABLE: u16 = 0xFFFF;


pub struct Memory {
    cartridge_rom: Rom,
    vram: Ram,
    eram: Ram,
    wram: Ram,
    echo: Ram,
    oam: Ram,
    io: Ram,
    hram: Ram,
    interupt_switch: u8
}

impl Memory {
    pub fn new(filepath: &str) -> Memory {
        Memory {
            cartridge_rom: Rom::new(filepath),
            vram: Ram::new(0x8000),
            eram: Ram::new(0x8000),
            wram: Ram::new(0x2000),
            echo: Ram::new(0x0), // TODO implement
            oam: Ram::new(0xA0),
            io: Ram::new(0x80),
            hram: Ram::new(0x7F),
            interupt_switch: 0
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let output = match address {
            ROM_START ..= ROM_END => self.cartridge_rom.read(address),
            VRAM_START ..= VRAM_END => self.vram.read(address - VRAM_START),
            ERAM_START ..= ERAM_END => self.eram.read(address - ERAM_START),
            WRAM_START ..= WRAM_END => self.wram.read(address - WRAM_START),
            ECHO_START ..= ECHO_END => self.echo.read(address - ECHO_START),
            OAM_START ..= OAM_END => self.oam.read(address - OAM_START),
            IO_START ..= IO_END => self.io.read(address - IO_START),
            HRAM_START ..= HRAM_END => self.hram.read(address - HRAM_START),
            INTERUPTS_ENABLE => self.interupt_switch,
            _ => panic!("Illegal read operation to address {:04X}", address)
        };
        //println!("read {:02X} from address {:04X}", output, address);
        output
    }

    pub fn write(&mut self, address: u16, value: u8) {
        // if true {//address == 0xFF02 && value == 0x81 {
        //     match str::from_utf8(&[value/* self.read(0xFF01) */]) {
        //         Ok(s) => print!("{}", s),
        //         _ => {}
        //     }
        // }
        //println!("writing {:02X} to address {:04X}", value, address);
        match address {
            ROM_START ..= ROM_END => {},//println!("writing {:02X} to ROM address {:04X}", value, address),
            VRAM_START ..= VRAM_END => self.vram.write(address - VRAM_START, value),
            ERAM_START ..= ERAM_END => self.eram.write(address - ERAM_START, value),
            WRAM_START ..= WRAM_END => self.wram.write(address - WRAM_START, value),
            ECHO_START ..= ECHO_END => self.echo.write(address - ECHO_START, value),
            OAM_START ..= OAM_END => self.oam.write(address - OAM_START, value),
            IO_START ..= IO_END => self.io.write(address - IO_START, value),
            HRAM_START ..= HRAM_END => self.hram.write(address - HRAM_START, value),
            INTERUPTS_ENABLE => self.interupt_switch = value,
            // TODO Interupts may need to be writable here
            _ => panic!("Illegal write operation to address {:04X}", address)
        }
    }

    // pub fn push_u8(&mut self, registers: &mut Registers, value: u8) {

    // }

    // pub fn pop_u8(&mut self, registers: &mut Registers) -> u8 {

    // }


    // TODO is the endianess correct here??
    pub fn push_u16(&mut self, regs: &mut Registers, value: u16) {
        let bytes = value.to_be_bytes();
        self.write(regs.sp, bytes[1]);
        self.write(regs.sp - 1, bytes[0]);
        regs.sp = regs.sp.wrapping_sub(2);
        //println!("push ${:04X}", value);
    }

    pub fn pop_u16(&mut self, regs: &mut Registers) -> u16 {
        let res = u16::from_be_bytes([self.read(regs.sp + 1), self.read(regs.sp + 2)]);
        regs.sp = regs.sp.wrapping_add(2);
        //println!("pop ${:04X}", res);
        res
    }

    pub fn print_cart_metadata(&self) {
        self.cartridge_rom.print_metadata();
    }
}

#[cfg(test)]
mod tests {
    use crate::gbc::memory::Registers;
    use crate::gbc::memory::Memory;

    #[test]
    fn test_stack_alloc() {
        let mut regs = Registers::new();
        let mut mem = Memory::new("./roms/tetris.gbc");
        mem.push_u16(&mut regs, 0x1234);
        assert_eq!(mem.pop_u16(&mut regs), 0x1234);
        mem.push_u16(&mut regs, 0xabcd);
        mem.push_u16(&mut regs, 0x9f9f);
        assert_eq!(mem.pop_u16(&mut regs), 0x9f9f);
        assert_eq!(mem.pop_u16(&mut regs), 0xabcd);
    }
}