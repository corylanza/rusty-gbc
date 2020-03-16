mod rom;
mod ram;

use rom::Rom;
use ram::Ram;

const ROM_START: u16 = 0;
const ROM_END: u16 = 0x3FFF;
const VRAM_START: u16 = 0x4000;
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
    interupt_switch: bool
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
            interupt_switch: true
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            ROM_START ..= ROM_END => self.cartridge_rom.read(address),
            VRAM_START ..= VRAM_END => self.vram.read(address - VRAM_START),
            ERAM_START ..= ERAM_END => self.eram.read(address - ERAM_START),
            WRAM_START ..= WRAM_END => self.wram.read(address - WRAM_START),
            ECHO_START ..= ECHO_END => self.echo.read(address - ECHO_START),
            OAM_START ..= OAM_END => self.oam.read(address - OAM_START),
            IO_START ..= IO_END => self.io.read(address - IO_START),
            HRAM_START ..= HRAM_END => self.hram.read(address - HRAM_START),
            INTERUPTS_ENABLE => self.interupt_switch as u8,
            _ => panic!("Illegal read operation to address {:04X}", address)
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            VRAM_START ..= VRAM_END => self.vram.write(address - VRAM_START, value),
            ERAM_START ..= ERAM_END => self.eram.write(address - ERAM_START, value),
            WRAM_START ..= WRAM_END => self.wram.write(address - WRAM_START, value),
            ECHO_START ..= ECHO_END => self.echo.write(address - ECHO_START, value),
            OAM_START ..= OAM_END => self.oam.write(address - OAM_START, value),
            IO_START ..= IO_END => self.io.write(address - IO_START, value),
            HRAM_START ..= HRAM_END => self.hram.write(address - HRAM_START, value),
            // TODO Interupts may need to be writable here
            _ => panic!("Illegal write operation to address {:04X}", address)
        }
    }
}