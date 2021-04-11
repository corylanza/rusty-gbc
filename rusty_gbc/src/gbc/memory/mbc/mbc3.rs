use super::MemoryBank;

pub struct MBC3 {
    rom_banks: Vec<Vec<u8>>,
    ram_banks: Vec<Vec<u8>>,
    selected_rom: u8,
    selected_ram: u8,
    ram_enabled: bool
}

impl MBC3 {
    pub fn load_rom(bytes: &Vec<u8>, save_bytes: &Vec<u8>, rom_bank_count: u16, ram_bank_count: u8, ram_bank_size: u16) -> MBC3 {
        println!("MBC3");
        let mut mbc = MBC3 {
            rom_banks: vec![vec![0; 0x4000]; rom_bank_count as usize],
            ram_banks: vec![vec![0; ram_bank_size as usize]; ram_bank_count as usize],
            selected_rom: 1,
            selected_ram: 0,
            ram_enabled: false,
        };
        println!("{} ROM banks of size 0x4000 (total {}Kbyte) {} RAM banks of size 0x{:04X} (total {}Kbyte)",
            rom_bank_count, mbc.rom_banks.len() / 0x400, ram_bank_count, ram_bank_size, (mbc.ram_banks.len() * ram_bank_size as usize) / 0x400);
        for (idx, byte) in bytes.iter().enumerate() {
            mbc.rom_banks[idx / 0x4000][idx % 0x4000] = *byte;
        }

        for (idx, byte) in save_bytes.iter().enumerate() {
            mbc.ram_banks[idx / ram_bank_size as usize][idx % ram_bank_size as usize] = *byte;
        }
        mbc
    }
}

impl MemoryBank for MBC3 {
    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0 ..= 0x1FFF => {
                self.ram_enabled = value & 0x0A == 0x0A;
            },
            0x2000 ..= 0x3FFF => {
                self.selected_rom = value & 0b01111111;
            },
            0x4000 ..= 0x5FFF => {
                self.selected_ram = value;
            },
            0x6000 ..= 0x7FFF => {
                //self.ram_banking_mode = value & 1 == 1;
            },
            _ => {}
        }
    }
    fn write_ram(&mut self, address: u16, value: u8) {
        let ram_bank_count = self.ram_banks.len();
        match self.ram_enabled {
            true => self.ram_banks[self.selected_ram as usize % ram_bank_count][address as usize] = value,
            false => {}
        }
    }
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0 ..= 0x3FFF => self.rom_banks[0][address as usize],
            0x4000 ..= 0x7FFF => {
                self.rom_banks[self.selected_rom as usize % self.rom_banks.len()][(address - 0x4000) as usize]
            },
            _ => panic!("ROM goes only to 0x7FFF, tried to read outside bounds")
        }
    }
    fn read_ram(&self, address: u16) -> u8 {
        match self.ram_enabled {
            true => self.ram_banks[self.selected_ram as usize % self.ram_banks.len()][address as usize],
            false => 0xFF
        }
    }
}