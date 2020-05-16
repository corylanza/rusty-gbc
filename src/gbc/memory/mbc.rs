use std::fs::File;
use std::io::prelude::*;
use std::str;

const MEMORY_BANK_TYPE_ADDRESS: u16 = 0x0147;
const TITLE_ADDRESS_MINUS_1: u16 = 0x0133;

pub trait MemoryBank {
    fn load_rom(&mut self, bytes: &Vec<u8>);
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

        println!("{}", str::from_utf8(&buffer[0x0134..0x0143]).unwrap());

        let mut mbc = Box::new(match buffer[MEMORY_BANK_TYPE_ADDRESS as usize] {
            0x00 => NoMBC { 
                rom: [0; 0x8000],
                ram: [0; 0x2000]
                },
            //0x01 => ,
            _ => panic!("not implemented")
        });
        mbc.load_rom(&buffer);
        mbc
    }

    pub fn print_metadata(&self) {
        //let bytes: Vec<u8> = (1..16).map(|x| self.read_rom(TITLE_ADDRESS_MINUS_1 + x)).value.collect();
        //println!("Title {}", str::from_utf8(&bytes).unwrap());
    }
}


struct NoMBC {
    rom: [u8; 0x8000],
    ram: [u8; 0x2000]
}

impl MemoryBank for NoMBC {
    fn load_rom(&mut self, bytes: &Vec<u8>) {
        for (idx, byte) in bytes.iter().enumerate() {
            self.rom[idx] = *byte;
        }
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {

    }
    fn write_ram(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }
    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize].wrapping_add(97)
    }
    fn read_ram(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }
}

struct MBC1 {
    rom_banks: [[u8; 0x4000]; 0x80],
    ram_banks: [[u8; 0x2000]; 4],
    selected_rom: u8,
    ram_enabled: bool,
    /// ROM banking mode if false, RAM banking mode if true
    ram_banking_mode: bool,
    selected_ram: u8,
}

impl MemoryBank for MBC1 {
    fn load_rom(&mut self, bytes: &Vec<u8>) {

    }

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
                    self.selected_ram = value & 0b00000011;
                } else {
                    panic!("ROM bank upper 2 bits not implemented")
                }
            },
            0x6000 ..= 0x7FFF => {
                self.ram_banking_mode = value & 1 == 1;
            },
            _ => {}
        }
    }
    fn write_ram(&mut self, address: u16, value: u8) {
        self.ram_banks[self.selected_ram as usize][address as usize] = value
    }
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0 ..= 0x3FFF => self.rom_banks[0][address as usize],
            0x4000 ..= 0x7FFF => self.rom_banks[self.selected_rom as usize][address as usize],
            _ => panic!("ROM goes only to 0x7FFF, tried to read outside bounds")
        }
    }
    fn read_ram(&self, address: u16) -> u8 {
        self.ram_banks[self.selected_ram as usize][address as usize]
    }
}