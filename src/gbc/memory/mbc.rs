use std::fs::File;
use std::io::prelude::*;
use std::str;

const MEMORY_BANK_TYPE_ADDRESS: u16 = 0x0147;
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
        match mbc_type {
            0 => NoMBC::load_rom(&buffer),
            1..=3 => Box::new(MBC1::load_rom(&buffer)),
            _ => panic!("not implemented {}", mbc_type)
        }
    }

    pub fn print_metadata(&self) {
        let bytes: Vec<u8> = (1..16).map(|x| self.read_rom(TITLE_ADDRESS_MINUS_1 + x)).collect();
        println!("{}", str::from_utf8(&bytes).unwrap());
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
    rom_banks: Vec<u8>,//[[u8; 0x4000]; 0x80],
    ram_banks: Vec<u8>,//[[u8; 0x2000]; 4],
    selected_rom: u8,
    ram_enabled: bool,
    /// ROM banking mode if false, RAM banking mode if true
    ram_banking_mode: bool,
    selected_ram: u8,
}

impl MBC1 {
    fn load_rom(bytes: &Vec<u8>) -> MBC1 {
        println!("MBC1");
        let mut mbc = MBC1 {
            rom_banks: vec![0; 0x4000 * 0x80],//[[0; 0x4000]; 0x80],
            ram_banks: vec![0; 0x2000 * 4],//[[0; 0x2000]; 4],
            selected_rom: 0,
            ram_enabled: false,
            ram_banking_mode: false,
            selected_ram: 0,
        };
        for (idx, byte) in bytes.iter().enumerate() {
            mbc.rom_banks[idx] = *byte;
        }
        mbc
    }
}

impl MemoryBank for MBC1 {
    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0 ..= 0x1FFF => {
                self.ram_enabled = value & 0x0A == 0x0A;
            },
            0x2000 ..= 0x3FFF => {
                match value {
                    // TODO take into account banks start at 1
                    0 => self.selected_rom = 1,
                    0x20 => self.selected_rom = 0x21,
                    0x40 => self.selected_rom = 0x41,
                    0x60 => self.selected_rom = 0x61,
                    _ => self.selected_rom = value & 0b00011111
                }
            },
            0x4000 ..= 0x5FFF => {
                if self.ram_banking_mode {
                    self.selected_ram = (self.selected_rom & 0b00000011) | (value & 0b00000011);
                } else {
                    self.selected_rom = (self.selected_rom & 0b10011111) | ((value & 0b00000011) << 5)
                }
            },
            0x6000 ..= 0x7FFF => {
                self.ram_banking_mode = value & 1 == 1;
            },
            _ => {}
        }
    }
    fn write_ram(&mut self, address: u16, value: u8) {
        self.ram_banks[self.selected_ram as usize * 0x2000 + address as usize] = value
    }
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0 ..= 0x3FFF => self.rom_banks[address as usize],
            0x4000 ..= 0x7FFF => self.rom_banks[self.selected_rom as usize * 0x4000 + address as usize],
            _ => panic!("ROM goes only to 0x7FFF, tried to read outside bounds")
        }
    }
    fn read_ram(&self, address: u16) -> u8 {
        self.ram_banks[self.selected_ram as usize * 0x2000 + address as usize]
    }
}