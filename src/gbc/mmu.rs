use super::Registers;
use super::memory::rom::Rom;
use super::memory::ram::Ram;
use std::str;
use super::gpu::Gpu;
use super::input::Input;

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
pub const INTERUPT_REQUEST: u16 = 0xFF0F;


pub struct Mmu {
    boot_rom: Vec<u8>,
    cartridge_rom: Rom,
    pub gpu: Gpu,
    dma: Option<Dma>,
    eram: Ram,
    wram: Ram,
    pub input: Input,
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
            dma: None,
            eram: Ram::new(0x8000),
            wram: Ram::new(0x2000),
            input: Input::new(),
            io: Ram::new(0x80),
            hram: Ram::new(0x7F),
            interupt_switch: 0,
            booting: true
        }
    }

    pub fn mmu_step(&mut self, cycles: u8) {
        self.gpu.gpu_step(cycles);
        self.interupt_switch |= self.gpu.interrupts | self.input.interrupt;
        self.gpu.interrupts = 0;
        self.input.interrupt = 0;
        let dma = self.dma;
        match dma {
            Some(ref dma) => self.dma_step(*dma, cycles),
            None => {}
        };
    }

    pub fn read(&self, address: u16) -> u8 {
        let output = match address {
            0 ..= 0xFF if self.booting => self.boot_rom[address as usize],
            ROM_START ..= ROM_END => self.cartridge_rom.read(address),
            VRAM_START ..= VRAM_END => self.gpu.read_from_vram(address - VRAM_START),
            ERAM_START ..= ERAM_END => self.eram.read(address - ERAM_START),
            WRAM_START ..= WRAM_END => self.wram.read(address - WRAM_START),
            ECHO_START ..= ECHO_END => self.wram.read(address - ECHO_START),
            OAM_START ..= OAM_END => self.gpu.read_from_oam(address - OAM_START),
            0xFEA0 ..= 0xFEFF => 0xFF, // Unusable returns this
            IO_START => self.input.read_joypad(),
            0xFF40 => self.gpu.lcd_control,
            0xFF41 => self.gpu.lcdc_status,
            0xFF42 => self.gpu.scy,
            0xFF43 => self.gpu.scx,
            0xFF44 => self.gpu.ly,
            0xFF45 => self.gpu.lyc,
            0xFF46 => match &self.dma {
                Some(dma) => dma.value,
                None => 0
            },
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
            OAM_START ..= OAM_END => self.gpu.write_to_oam(address - OAM_START, value),
            0xFEA0 ..= 0xFEFF => { /* Unusable */} ,
            0xFF00 => self.input.write_joypad(value),
            // 0xFF01 SB serial transfer data
            // 0xFF02 SC serial transfer control
            // 0xFF04 DIV divider register
            // 0xFF05 TIMA timer counter
            // 0xFF06 TMA timer modulo
            // 0xFF07 TAC timer control
            0xFF40 => self.gpu.lcd_control = value,
            0xFF41 => self.gpu.lcdc_status = value & 0b11111000,
            0xFF42 => self.gpu.scy= value,
            0xFF43 => self.gpu.scx = value,
            0xFF44 => { /* No Writes to VRAM*/},
            0xFF45 => self.gpu.lyc = value,
            0xFF46 => { self.dma = Some(Dma::new(value)); },
            // 0xFF47 => self.gpu.bgp = value,
            // 0xFF48 => self.gpu.obp0 = value,
            // 0xFF49 => self.gpu.obp1 = value,
            0xFF4A => self.gpu.wy = value,
            0xFF4B => self.gpu.wx = value,
            // 0xFF51 cgb hdma1
            // 0xFF52 cgb hdma2
            // 0xFF53 cgb hdma3
            // 0xFF54 cgb hdma4
            // 0xFF55 cgb hdma5
            // 0xFF68 cgb bgpi
            // 0xFF69 cgb pgpd
            // 0xFF6A cgb spi
            // 0xFF6a cgb spd
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

    fn dma_step(&mut self, mut dma: Dma, cycles: u8) {
        for _ in 0 ..= (cycles / 4) {
            if dma.started && dma.address < 0xA0 {
                let val = self.read(dma.source + dma.address as u16);
                self.write(OAM_START + dma.address as u16, val);
            } else if dma.started {
                self.dma = None;
                return
            } else {
                dma.started = true;
            }
            dma.address += 4;
        }
        self.dma = Some(dma);
    }
}

#[derive(Copy, Clone)]
struct Dma {
    value: u8,
    source: u16,
    address: u8,
    started: bool
}

impl Dma {
    fn new(value: u8) -> Self {
        Dma {
            value: value,
            source: (value as u16) << 8,
            address: 0,
            started: false
        }
    }
}