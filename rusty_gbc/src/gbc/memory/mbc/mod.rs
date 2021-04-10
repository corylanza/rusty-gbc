use std::str;

mod mbc1;
mod mbc5;

use mbc1::MBC1;
use mbc5::MBC5;

const MEMORY_BANK_TYPE_ADDRESS: u16 = 0x0147;
const ROM_SIZE_ADDRESS: u16 = 0x0148;
const RAM_SIZE_ADDRESS: u16 = 0x0149;
const TITLE_ADDRESS_MINUS_1: u16 = 0x0133;

pub trait MemoryBank {
    fn write_rom(&mut self, address: u16, value: u8);
    fn write_ram(&mut self, address: u16, value: u8);
    fn read_rom(&self, address: u16) -> u8;
    fn read_ram(&self, address: u16) -> u8;
}

impl dyn MemoryBank {
    pub fn new(rom_bytes: Vec<u8>, save_bytes: Vec<u8>) -> Box<dyn MemoryBank> {
        let mbc_type = rom_bytes[MEMORY_BANK_TYPE_ADDRESS as usize];
        let rom_size = rom_bytes[ROM_SIZE_ADDRESS as usize];
        let rom_bank_count: u16 = match rom_size {
            0 ..= 8 => ((0x8000 << rom_size as usize) / 0x4000) as u16,
            _ => panic!("Unsupported ROM size {:02X}", rom_size)
        };
        let ram_size = rom_bytes[RAM_SIZE_ADDRESS as usize];
        let (ram_bank_count, ram_bank_size) = match ram_size {
            0 => (0, 0),
            1 => (1, 0x800),
            2 => (1, 0x2000),
            3 => (4, 0x2000),
            4 => (16, 0x2000),
            5 => (8, 0x2000),
            _ => panic!("Unsupported RAM size {:02X}", ram_size)
        };
        match mbc_type {
            0 => NoMBC::load_rom(&rom_bytes),
            1 ..= 3 => Box::new(MBC1::load_rom(&rom_bytes, &save_bytes, rom_bank_count, ram_bank_count, ram_bank_size)),
            0x19 ..= 0x1E => Box::new(MBC5::load_rom(&rom_bytes, &save_bytes, rom_bank_count, ram_bank_count, ram_bank_size)),
            _ => panic!("not implemented {}", mbc_type)
        }
    }

    pub fn print_metadata(&self) {
        let bytes: Vec<u8> = (1..16).map(|x| self.read_rom(TITLE_ADDRESS_MINUS_1 + x)).collect();
        println!("{}", str::from_utf8(&bytes).unwrap());
        // match &self.bytes[0x0147] {
        //     0x00 => println!("ROM ONLY"),
        //     0x01 => println!("MBC 1"),
        //     0x02 => println!("MBC1+RAM"),
        //     0x03 => println!("MBC1+RAM+BATTERY"),
        //     0x05 => println!("MBC2"),
        //     0x06 => println!("MBC2+BATTERY"),
        //     0x08 => println!("ROM+RAM"),
        //     0x09 => println!("ROM+RAM+BATTERY"),
        //     0x0B => println!("MMM01"),
        //     0x0C => println!("MMM01+RAM"),
        //     0x0D => println!("MMM01+RAM+BATTERY"),
        //     0x0F => println!("MBC3+TIMER+BATTERY"),
        //     0x10 => println!("MBC3+TIMER+RAM+BATTERY"),
        //     0x11 => println!("MBC3"),
        //     0x12 => println!("MBC3+RAM"),
        //     0x13 => println!("MBC3+RAM+BATTERY"),
        //     0x19 => println!("MBC5"),
        //     0x1A => println!("MBC5+RAM"),
        //     0x1B => println!("MBC5+RAM+BATTERY"),
        //     0x1C => println!("MBC5+RUMBLE"),
        //     0x1D => println!("MBC5+RUMBLE+RAM"),
        //     0x1E => println!("MBC5+RUMBLE+RAM+BATTERY"),
        //     0x20 => println!("MBC6"),
        //     0x22 => println!("MBC7+SENSOR+RUMBLE+RAM+BATTERY"),
        //     _ => println!("unrecognized cart type")
    }
}


struct NoMBC {
    rom: [u8; 0x8000],
    ram: [u8; 0x2000]
}

impl NoMBC {
    fn load_rom(bytes: &Vec<u8>) -> Box<dyn MemoryBank> {
        let mut mbc = Box::new(NoMBC { 
            rom: [0; 0x8000],
            ram: [0; 0x2000]
        });
        for (idx, byte) in bytes.iter().enumerate() {
            mbc.rom[idx] = *byte;
        }
        mbc
    }
}

impl MemoryBank for NoMBC {
    fn write_rom(&mut self, _address: u16, _value: u8) {

    }
    fn write_ram(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }
    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }
    fn read_ram(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }
}