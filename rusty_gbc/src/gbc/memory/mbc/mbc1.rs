use super::MemoryBank;

pub struct MBC1 {
    rom_banks: Vec<Vec<u8>>,
    ram_banks: Vec<Vec<u8>>,
    selected_rom: u8,
    two_bits: u8,
    ram_enabled: bool,
    /// ROM banking mode if false, RAM banking mode if true
    ram_banking_mode: bool,
}

impl MBC1 {
    pub fn load_rom(bytes: &Vec<u8>, rom_bank_count: u16, ram_bank_count: u8, ram_bank_size: u16) -> MBC1 {
        println!("MBC1");
        //Special limitation of MBC1
        let rom_bank_count = match rom_bank_count {
            64 => 63,
            128 => 125,
            129 ..= 0xFFFF => panic!("MBC1 does not support {} rom banks", rom_bank_count),
            _ => rom_bank_count
        };
        let mut mbc = MBC1 {
            rom_banks: vec![vec![0; 0x4000]; rom_bank_count as usize],
            ram_banks: vec![vec![0; ram_bank_size as usize]; ram_bank_count as usize],
            selected_rom: 1,
            two_bits: 0,
            ram_enabled: false,
            ram_banking_mode: false,
        };
        println!("{} ROM banks of size 0x4000 (total {}Kbyte) {} RAM banks of size 0x{:04X} (total {}Kbyte)",
            rom_bank_count, mbc.rom_banks.len() / 0x400, ram_bank_count, ram_bank_size, (mbc.ram_banks.len() * ram_bank_size as usize) / 0x400);
        for (idx, byte) in bytes.iter().enumerate() {
            mbc.rom_banks[idx / 0x4000][idx % 0x4000] = *byte;
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
                //println!("ram enabled: {}", self.ram_enabled);
            },
            0x2000 ..= 0x3FFF => {
                self.selected_rom = match value & 0b00011111 {
                    0 => (value + 1) & 0b00011111,
                    _ => value & 0b00011111
                };
                //println!("selected rom: {} with value: {}", self.selected_rom, value);
            },
            0x4000 ..= 0x5FFF => {
                self.two_bits = value & 0b11;
                //println!("two bits: {} value: {}", self.two_bits, value);
            },
            0x6000 ..= 0x7FFF => {
                self.ram_banking_mode = value & 1 == 1;
                //println!("ram banking: {}", self.ram_banking_mode);
            },
            _ => {}
        }
    }
    fn write_ram(&mut self, address: u16, value: u8) {
        let ram_bank_count = self.ram_banks.len();
        match self.ram_enabled {
            true if self.ram_banking_mode => self.ram_banks[self.two_bits as usize % ram_bank_count][address as usize] = value,
            true => self.ram_banks[0][address as usize] = value,
            false => {}
        }
    }
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0 ..= 0x3FFF => self.rom_banks[0][address as usize],
            0x4000 ..= 0x7FFF => {
                self.rom_banks[self.selected_rom() as usize % self.rom_banks.len()][(address - 0x4000) as usize]
            },
            _ => panic!("ROM goes only to 0x7FFF, tried to read outside bounds")
        }
    }
    fn read_ram(&self, address: u16) -> u8 {
        match self.ram_enabled {
            true if self.ram_banking_mode => self.ram_banks[self.two_bits as usize % self.ram_banks.len()][address as usize],
            true => self.ram_banks[0][address as usize],
            false => 0xFF
        }
    }
}