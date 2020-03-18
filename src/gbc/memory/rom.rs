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
        println!("{}", str::from_utf8(&self.bytes[0x0134..0x0143]).unwrap())
    }

    pub fn read(&self, address: u16) -> u8 {
        self.bytes[address as usize]
    }
}