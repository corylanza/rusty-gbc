extern crate rand;

use super::Registers;
use super::memory::ram::Ram;
use super::memory::mbc::MemoryBank;
use std::str;
use super::gpu::Gpu;
use super::input::Input;
use super::timer::Timer;

const ROM_START: u16 = 0;
const ROM_END: u16 = 0x7FFF;
const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const ERAM_START: u16 = 0xA000;
const ERAM_END: u16 = 0xBFFF;
const WRAM_BANK_0_START: u16 = 0xC000;
const WRAM_BANK_0_END: u16 = 0xCFFF;
const WRAM_BANK_1_START: u16 = 0xD000;
const WRAM_END: u16 = 0xDFFF;
const ECHO_START: u16 = 0xE000;
const ECHO_END: u16 = 0xFDFF;
const OAM_START: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;
const IO_START: u16 = 0xFF00;
const IO_END: u16 = 0xFF7F;
const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;
pub const INTERUPTS_ENABLE: u16 = 0xFFFF;
pub const INTERUPT_REQUEST: u16 = 0xFF0F;


pub struct Mmu {
    boot_rom: Vec<u8>,
    pub gpu: Gpu,
    dma: Option<Dma>,
    hdma: Option<Hdma>,
    mbc: Box<dyn MemoryBank>,
    wram: Vec<Ram>,
    pub input: Input,
    timer: Timer,
    io: Ram,
    hram: Ram,
    interupt_switch: u8,
    wram_select: u8,
    pub booting: bool
}

impl Mmu {
    pub fn new(rom_bytes: Vec<u8>, gpu: Gpu) -> Mmu {
        if gpu.color_mode {
            println!("Color");
        }
        
        let mbc = MemoryBank::new(rom_bytes);
        let mut wram = Vec::new();
        for _ in 0 .. if gpu.color_mode { 8 } else { 2 } {   
            wram.push(Ram::new(0x2000));
        }

        let boot_rom = if gpu.color_mode { super::boot::load_cgb_rom() } else { super::boot::load_rom() };

        Mmu {
            boot_rom,
            mbc,
            gpu,
            dma: None,
            hdma: None,
            wram: wram,
            input: Input::new(),
            timer: Timer::new(),
            io: Ram::new(0x80),
            hram: Ram::new(0x7F),
            interupt_switch: 0,
            wram_select: 0,
            booting: true
        }
    }

    pub fn mmu_step(&mut self, cycles: u8) {
        let int = self.read(INTERUPT_REQUEST) | self.gpu.interrupts | self.input.interrupt | self.timer.interrupt;
        self.write(INTERUPT_REQUEST, int);
        self.gpu.interrupts = 0;
        self.input.interrupt = 0;
        self.timer.interrupt = 0;
        self.timer.timer_step(cycles);
        let dma = self.dma;
        match dma {
            Some(ref dma) => self.dma_step(*dma, cycles),
            None => {}
        };
    }

    #[allow(overlapping_patterns)]
    pub fn read(&self, address: u16) -> u8 {

        if address == 0x42 {
            println!("{:04X}", address);
        }

        let output = match address {
            0 ..= 0x8FF if self.booting && self.gpu.color_mode => self.boot_rom[address as usize],
            0 ..= 0xFF if self.booting => self.boot_rom[address as usize],
            ROM_START ..= ROM_END => self.mbc.read_rom(address),
            VRAM_START ..= VRAM_END => self.gpu.read_from_vram(address - VRAM_START),
            ERAM_START ..= ERAM_END => self.mbc.read_ram(address - ERAM_START),
            WRAM_BANK_0_START ..= WRAM_BANK_0_END => self.wram[0].read(address - WRAM_BANK_0_START),
            WRAM_BANK_1_START ..= WRAM_END if self.gpu.color_mode => self.wram[self.wram_select as usize].read(address - WRAM_BANK_1_START),
            WRAM_BANK_1_START ..= WRAM_END => self.wram[1].read(address - WRAM_BANK_1_START),
            ECHO_START ..= ECHO_END => self.wram[((address - ECHO_START) / 0x2000) as usize].read(address - ECHO_START),
            OAM_START ..= OAM_END => self.gpu.read_from_oam(address - OAM_START),
            0xFEA0 ..= 0xFEFF => 0xFF, // Unusable returns this
            IO_START => self.input.read_joypad(),
            0xFF04 => self.timer.get_div(),
            0xFF05 => self.timer.tima,
            0xFF06 => self.timer.tma,
            0xFF07 => self.timer.get_timer_control(),
            0xFF40 => self.gpu.get_lcdc_control(),
            0xFF41 => self.gpu.get_lcdc_status(),
            0xFF42 => self.gpu.get_scy(),
            0xFF43 => self.gpu.get_scx(),
            0xFF44 => self.gpu.get_ly(),
            0xFF45 => self.gpu.get_lyc(),
            0xFF46 => match &self.dma {
                Some(dma) => dma.value,
                None => 0
            },
            0xFF47 => self.gpu.get_bgp(),
            0xFF48 => self.gpu.get_obp0(),
            0xFF49 => self.gpu.get_obp1(),
            0xFF4A => self.gpu.get_wy(),
            0xFF4B => self.gpu.get_wx(),
            0xFF4F => self.gpu.get_vram_bank(),
            // 0xFF50 => boot rom enabled
            0xFF51 => 0xFF, // HDMA1 High Source byte (write only),
            0xFF52 => 0xFF, // HDMA2 Low Source byte (write only),
            0xFF53 => 0xFF, // HDMA3 High dest byte (write only),
            0xFF54 => 0xFF, // HDMA4 Low dest byte (write only),
            //0xFF55 => self.hdma.unwrap().value, // HDMA5 Length/mode/start (write only),
            0xFF68 => self.gpu.get_color_bg_palette_idx(),//cgb bgpi
            0xFF69 => self.gpu.get_color_bg_palette(),//cgb pgpd
            0xFF6A => self.gpu.get_color_sprite_palette_idx(), //cgb spi
            0xFF6B => self.gpu.get_color_sprite_palette(), //cgb spd
            0xFF70 => self.wram_select | 0b11111000, // TODO verify
            IO_START ..= IO_END => self.io.read(address - IO_START),
            HRAM_START ..= HRAM_END => self.hram.read(address - HRAM_START),
            INTERUPTS_ENABLE => self.interupt_switch
        };
        
        output
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read(address), self.read(address + 1)])
    }

    #[allow(overlapping_patterns)]
    pub fn write(&mut self, address: u16, value: u8) {
        if address == 0xFF02 && value == 0x81 {
            match str::from_utf8(&[self.read(0xFF01)]) {
                Ok(s) => print!("{}", s),
                _ => { }
            }
        }
        if self.booting && address == 0xFF50 {
            self.booting = false;
            self.mbc.print_metadata();
            println!("boot complete");
        }

        if address == 0xff4f {
            println!("{:04X} {:02X}", address, value);
        }

        match address {
            ROM_START ..= ROM_END => self.mbc.write_rom(address, value),
            VRAM_START ..= VRAM_END => self.gpu.write_to_vram(address - VRAM_START, value),
            ERAM_START ..= ERAM_END => self.mbc.write_ram(address - ERAM_START, value),
            WRAM_BANK_0_START ..= WRAM_BANK_0_END => self.wram[0].write(address - WRAM_BANK_0_START, value),
            WRAM_BANK_1_START ..= WRAM_END if self.gpu.color_mode => self.wram[self.wram_select as usize].write(address - WRAM_BANK_1_START, value),
            WRAM_BANK_1_START ..= WRAM_END => self.wram[1].write(address - WRAM_BANK_1_START, value),
            ECHO_START ..= ECHO_END => self.wram[((address - ECHO_START) / 0x2000) as usize].write(address - ECHO_START, value),
            OAM_START ..= OAM_END => self.gpu.write_to_oam(address - OAM_START, value),
            0xFEA0 ..= 0xFEFF => { /* Unusable */} ,
            0xFF00 => self.input.write_joypad(value),
            // 0xFF01 SB serial transfer data
            // 0xFF02 SC serial transfer control
            0xFF04 => self.timer.reset_div(), // writing any value to DIV resets it to 0
            0xFF05 => self.timer.tima = value,
            0xFF06 => self.timer.tma = value,
            0xFF07 => self.timer.set_timer_control(value),
            0xFF40 => self.gpu.set_lcdc_control(value),
            0xFF41 => self.gpu.set_lcdc_status(value),
            0xFF42 => self.gpu.set_scy(value),
            0xFF43 => self.gpu.set_scx(value),
            0xFF44 => { /* No Writes to VRAM*/},
            0xFF45 => self.gpu.set_lyc(value),
            0xFF46 => { self.dma = Some(Dma::new(value)); },
            0xFF47 => self.gpu.set_bgp(value),
            0xFF48 => self.gpu.set_obp0(value),
            0xFF49 => self.gpu.set_obp1(value),
            0xFF4A => self.gpu.set_wy(value),
            0xFF4B => self.gpu.set_wx(value),
            0xFF4D => panic!("Double speed not implemented"),
            0xFF4F => self.gpu.select_vram_bank(value),
            0xFF51 => {}, // HDMA1 High Source byte (write only),
            0xFF52 => {}, // HDMA2 Low Source byte (write only),
            0xFF53 => {}, // HDMA3 High dest byte (write only),
            0xFF54 => {}, // HDMA4 Low dest byte (write only),
            0xFF55 => panic!("HDMA not implemented"),//self.hdma.unwrap().value, // HDMA5 Length/mode/start (write only),
            0xFF68 => self.gpu.set_color_bg_palette_idx(value),//cgb bgpi
            0xFF69 => self.gpu.set_color_bg_palette(value),//cgb pgpd
            0xFF6A => self.gpu.set_color_sprite_palette_idx(value), //cgb spi
            0xFF6B => self.gpu.set_color_sprite_palette(value), //cgb spd
            0xFF70 => {
                self.wram_select = if value == 1 { 1 } else { value & 0b00000111 }; // TODO verify
                println!("selected {} with value {}", self.wram_select, value);
            },
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
                //println!("{:04X} to {:04X}", dma.source + dma.address as u16, OAM_START + dma.address as u16);
                let val = self.read(dma.source + dma.address as u16);
                // TODO writes to OAM need to overide write
                self.write(OAM_START + dma.address as u16, val);
                dma.address += 1;
            } else if dma.started {
                self.dma = None;
                return
            } else {
                dma.started = true;
            }
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

struct Hdma {
    value: u8,
    source: u16,
    destination: u16,
    length: u8
}