use super::MemoryBank;

pub struct MBC5 {
    rom_banks: Vec<Vec<u8>>,
    ram_banks: Vec<Vec<u8>>,
    selected_rom: u16,
    selected_ram: u8,
    ram_enabled: bool,
}

impl MBC5 {
    pub fn load_rom(bytes: &Vec<u8>, save_bytes: &Vec<u8>, rom_bank_count: u16, ram_bank_count: u8, ram_bank_size: u16) -> MBC5 {
        println!("MBC5");
        let rom_bank_count = match rom_bank_count {
            0x1E0 ..= 0xFFFF => panic!("MBC5 does not support {} rom banks", rom_bank_count),
            _ => rom_bank_count
        };
        let mut mbc = MBC5 {
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

impl MemoryBank for MBC5 {
    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0 ..= 0x1FFF => {
                self.ram_enabled = value & 0x0A == 0x0A;
                //println!("ram enabled: {}", self.ram_enabled);
            },
            0x2000 ..= 0x2FFF => {
                self.selected_rom = u16::from_be_bytes([(self.selected_rom >> 8) as u8, value]);
                //println!("selected rom: {} with value: {}", self.selected_rom, value);
            },
            0x3000 ..= 0x3FFF => {
                self.selected_rom = u16::from_be_bytes([value & 1, (self.selected_rom & 0xFF) as u8]);
            },
            0x4000 ..= 0x5FFF => {
                self.selected_ram = value & 0xF;
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