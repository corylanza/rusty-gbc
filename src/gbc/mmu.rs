use super::Registers;
use super::memory::rom::Rom;
use super::memory::ram::Ram;
use std::str;
use super::gpu::Gpu;

const ROM_START: u16 = 0;
const ROM_END: u16 = 0x7FFF;
const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const ERAM_START: u16 = 0xA000;
const ERAM_END: u16 = 0xBFFF;
const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xDFFF;
const ECHO_START: u16 = 0xE000;
const ECHO_END: u16 = 0xFDFF;
const OAM_START: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;
const IO_START: u16 = 0xFF00;
const IO_END: u16 = 0xFF7F;
const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;
const INTERUPTS_ENABLE: u16 = 0xFFFF;
const INTERUPT_REQUEST: u16 = 0xFF0F;


pub struct Mmu {
    boot_rom: Vec<u8>,
    cartridge_rom: Rom,
    pub gpu: Gpu,
    eram: Ram,
    wram: Ram,
    oam: Ram,
    io: Ram,
    hram: Ram,
    interupt_switch: u8,
    pub booting: bool
}

impl Mmu {
    pub fn new(filepath: &str, gpu: Gpu) -> Mmu {
        Mmu {
            boot_rom: super::boot::load_rom(),
            cartridge_rom: Rom::new(filepath),
            gpu: gpu,
            eram: Ram::new(0x8000),
            wram: Ram::new(0x2000),
            oam: Ram::new(0xA0),
            io: Ram::new(0x80),
            hram: Ram::new(0x7F),
            interupt_switch: 0,
            booting: true
        }
    }

    pub fn mmu_step(&mut self, cycles: u8) {
        self.gpu.gpu_step(cycles);
        self.write(INTERUPT_REQUEST, self.read(INTERUPT_REQUEST) | self.gpu.interrupts);
        self.gpu.interrupts = 0;
    }

    pub fn read(&self, address: u16) -> u8 {
        let output = match address {
            0 ..= 0xFF if self.booting => self.boot_rom[address as usize],
            ROM_START ..= ROM_END => self.cartridge_rom.read(address),
            VRAM_START ..= VRAM_END => self.gpu.read_from_vram(address - VRAM_START),
            ERAM_START ..= ERAM_END => self.eram.read(address - ERAM_START),
            WRAM_START ..= WRAM_END => self.wram.read(address - WRAM_START),
            ECHO_START ..= ECHO_END => self.wram.read(address - ECHO_START),
            OAM_START ..= OAM_END => self.oam.read(address - OAM_START),
            0xFF00 => { 0x00 },
            0xFEA0 ..= 0xFEFF => 0xFF, // Unusable returns this
            0xFF40 => self.gpu.lcd_control,
            0xFF41 => self.gpu.lcdc_status,
            0xFF42 => self.gpu.scy,
            0xFF43 => self.gpu.scx,
            0xFF44 => self.gpu.ly,
            0xFF45 => self.gpu.lyc,
            0xFF4A => self.gpu.wy,
            0xFF4B => self.gpu.wx,
            IO_START ..= IO_END => self.io.read(address - IO_START),
            HRAM_START ..= HRAM_END => self.hram.read(address - HRAM_START),
            INTERUPTS_ENABLE => self.interupt_switch
        };
        //println!("read {:02X} from address {:04X}", output, address);
        output
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read(address), self.read(address + 1)])
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address == 0xFF02 && value == 0x81 {
            match str::from_utf8(&[self.read(0xFF01)]) {
                Ok(s) => print!("{}", s),
                _ => { }
            }
        }
        if self.booting && address == 0xFF50 {
            println!("boot complete");
            self.booting = false;
            self.cartridge_rom.print_metadata();
        }
        //println!("writing {:02X} to address {:04X}", value, address);
        match address {
            ROM_START ..= ROM_END => {},//println!("writing {:02X} to ROM address {:04X}", value, address),
            VRAM_START ..= VRAM_END => self.gpu.write_to_vram(address - VRAM_START, value),
            ERAM_START ..= ERAM_END => self.eram.write(address - ERAM_START, value),
            WRAM_START ..= WRAM_END => self.wram.write(address - WRAM_START, value),
            ECHO_START ..= ECHO_END => self.wram.write(address - ECHO_START, value),
            OAM_START ..= OAM_END => self.oam.write(address - OAM_START, value),
            0xFF00 => { /* Can not write to joypad register */ },
            0xFEA0 ..= 0xFEFF => { /* Unusable */} ,
            0xFF40 => self.gpu.lcd_control = value,
            0xFF41 => self.gpu.lcdc_status = value & 0b11111000,
            0xFF42 => self.gpu.scy= value,
            0xFF43 => self.gpu.scx = value,
            0xFF44 => { /* No Writes to VRAM*/},
            0xFF45 => self.gpu.lyc = value,
            0xFF4A => self.gpu.wy = value,
            0xFF4B => self.gpu.wx = value,
            IO_START ..= IO_END => self.io.write(address - IO_START, value),
            HRAM_START ..= HRAM_END => self.hram.write(address - HRAM_START, value),
            INTERUPTS_ENABLE => self.interupt_switch = value,
        }
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        let bytes = value.to_le_bytes();
        self.write(address, bytes[0]);
        self.write(address + 1, bytes[1]);
    }

    pub fn push_u16(&mut self, regs: &mut Registers, value: u16) {
        regs.sp = regs.sp.wrapping_sub(2);
        self.write_u16(regs.sp, value);
    }

    pub fn pop_u16(&mut self, regs: &mut Registers) -> u16 {
        let res = self.read_u16(regs.sp);
        regs.sp = regs.sp.wrapping_add(2);
        res
    }
}