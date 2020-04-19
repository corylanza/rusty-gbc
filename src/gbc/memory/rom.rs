use std::fs::File;
use std::io::prelude::*;
use std::str;

pub struct Rom {
    bytes: Vec<u8>
}

impl Rom {
    pub fn new(filepath: &str) -> Rom {
        let mut file = File::open(&filepath).unwrap();
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).unwrap();

        Rom {
            bytes: buffer
        }
    }

    pub fn print_metadata(&self) {
        println!("{}", str::from_utf8(&self.bytes[0x0134..0x0143]).unwrap());
        match &self.bytes[0x0143] {
            0x80 => println!("CGB and DMG"),
            0xC0 => println!("CGB only"),
            _ => {}
        };
        match &self.bytes[0x0147] {
            0x00 => println!("ROM ONLY"),
            0x01 => println!("MBC 1"),
            0x02 => println!("MBC1+RAM"),
            0x03 => println!("MBC1+RAM+BATTERY"),
            0x05 => println!("MBC2"),
            0x06 => println!("MBC2+BATTERY"),
            0x08 => println!("ROM+RAM"),
            0x09 => println!("ROM+RAM+BATTERY"),
            0x0B => println!("MMM01"),
            0x0C => println!("MMM01+RAM"),
            0x0D => println!("MMM01+RAM+BATTERY"),
            0x0F => println!("MBC3+TIMER+BATTERY"),
            0x10 => println!("MBC3+TIMER+RAM+BATTERY"),
            0x11 => println!("MBC3"),
            0x12 => println!("MBC3+RAM"),
            0x13 => println!("MBC3+RAM+BATTERY"),
            0x19 => println!("MBC5"),
            0x1A => println!("MBC5+RAM"),
            0x1B => println!("MBC5+RAM+BATTERY"),
            0x1C => println!("MBC5+RUMBLE"),
            0x1D => println!("MBC5+RUMBLE+RAM"),
            0x1E => println!("MBC5+RUMBLE+RAM+BATTERY"),
            0x20 => println!("MBC6"),
            0x22 => println!("MBC7+SENSOR+RUMBLE+RAM+BATTERY"),
            _ => println!("unrecognized cart type")
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.bytes[address as usize]
    }
}