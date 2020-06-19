use std::fs::File;
use std::io::prelude::*;
use std::str;

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
    pub fn new(filepath: &str) -> Box<dyn MemoryBank> {
        let mut file = File::open(&filepath).unwrap();
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).unwrap();

        let mbc_type = buffer[MEMORY_BANK_TYPE_ADDRESS as usize];
        let rom_size = buffer[ROM_SIZE_ADDRESS as usize];
        let rom_bank_count = match rom_size {
            0 => 0,
            1 ..= 8 => 2u8.pow(rom_size as u32 + 1),
            _ => panic!("Unsupported ROM size {:02X}", rom_size)
        };
        let ram_size = buffer[RAM_SIZE_ADDRESS as usize];
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
            0 => NoMBC::load_rom(&buffer),
            1..=3 => Box::new(MBC1::load_rom(&buffer, rom_bank_count, ram_bank_count, ram_bank_size)),
            _ => panic!("not implemented {}", mbc_type)
        }
    }

    pub fn print_metadata(&self) {
        let bytes: Vec<u8> = (1..16).map(|x| self.read_rom(TITLE_ADDRESS_MINUS_1 + x)).collect();
        println!("{}", str::from_utf8(&bytes).unwrap());
        // match &self.bytes[0x0143] {
        //     0x80 => println!("CGB and DMG"),
        //     0xC0 => println!("CGB only"),
        //     _ => {}
        // };
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

struct MBC1 {
    rom_banks: Vec<u8>,
    ram_banks: Vec<u8>,
    selected_rom: u8,
    two_bits: u8,
    ram_enabled: bool,
    /// ROM banking mode if false, RAM banking mode if true
    ram_banking_mode: bool,
}

impl MBC1 {
    fn load_rom(bytes: &Vec<u8>, rom_bank_count: u8, ram_bank_count: u8, ram_bank_size: u16) -> MBC1 {
        println!("MBC1");
        //Special limitation of MBC1
        let rom_bank_count = match rom_bank_count {
            64 => 63,
            128 => 125,
            _ => rom_bank_count
        };
        let mut mbc = MBC1 {
            rom_banks: vec![0; 0x4000 * rom_bank_count as usize],
            ram_banks: vec![0; ram_bank_size as usize * ram_bank_count as usize],
            selected_rom: 0,
            two_bits: 0,
            ram_enabled: false,
            ram_banking_mode: false,
        };
        println!("{} ROM banks of size 0x4000 (total {}Kbyte) {} RAM banks of size 0x{:04X} (total {}Kbyte)",
            rom_bank_count, mbc.rom_banks.len() / 0x400, ram_bank_count, ram_bank_size, mbc.ram_banks.len() / 0x400);
        for (idx, byte) in bytes.iter().enumerate() {
            mbc.rom_banks[idx] = *byte;
        }
        mbc
    }

    fn selected_rom(&self) -> u8 {
        match self.ram_banking_mode {
            true => self.selected_rom,
            false => self.selected_rom | (self.two_bits << 5)
        }
    }
}

impl MemoryBank for MBC1 {
    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0 ..= 0x1FFF => {
                self.ram_enabled = value & 0x0A == 0x0A;
            },
            0x2000 ..= 0x3FFF => {
                self.selected_rom = match value & 0b00011111 {
                    0 => (value + 1) & 0b00011111,
                    _ => value & 0b00011111
                }
            },
            0x4000 ..= 0x5FFF => {
                self.two_bits = value & 0b11;
            },
            0x6000 ..= 0x7FFF => {
                self.ram_banking_mode = value & 1 == 1;
            },
            _ => {}
        }
    }
    fn write_ram(&mut self, address: u16, value: u8) {
        match self.ram_enabled {
            true if self.ram_banking_mode => self.ram_banks[self.two_bits as usize * 0x2000 + address as usize] = value,
            true => self.ram_banks[address as usize] = value,
            false => {}
        }
    }
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0 ..= 0x3FFF => self.rom_banks[address as usize],
            0x4000 ..= 0x7FFF => self.rom_banks[(self.selected_rom() as usize * 0x4000) + (address - 0x4000) as usize],
            _ => panic!("ROM goes only to 0x7FFF, tried to read outside bounds")
        }
    }
    fn read_ram(&self, address: u16) -> u8 {
        match self.ram_enabled {
            true if self.ram_banking_mode => self.ram_banks[(self.two_bits as usize * 0x2000) + address as usize],
            true => self.ram_banks[address as usize],
            false => 0xFF
        }
    }
}