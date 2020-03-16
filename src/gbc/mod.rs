mod memory;
mod registers;

use memory::Memory;
use registers::Registers;

pub struct Gameboy {
    memory: Memory,
    registers: Registers
}

impl Gameboy {
    pub fn new(cartridge_path: &str) -> Gameboy {
        Gameboy {
            memory: Memory::new(cartridge_path),
            registers: Registers::new()
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu_step();
        }
    }

    pub fn next_byte(&mut self) -> u8 {
        let byte = self.memory.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    pub fn next_u16(&mut self) -> u16 {
        u16::from_le_bytes([self.next_byte(), self.next_byte()])
    }


    pub fn cpu_step(&mut self) {
        let opcode = self.next_byte();
        println!("executing ${:02X} at address ${:04X}", opcode, self.registers.pc-1);

        match opcode {
            // LD B,n
            0x06 => { self.registers.b = self.next_byte(); },
            // LD C,n
            0x0E => { self.registers.c = self.next_byte(); },
            // LD D,n
            0x16 => { self.registers.d = self.next_byte(); },
            // LD E,n
            0x1E => { self.registers.e = self.next_byte(); },
            // LD H,n
            0x26 => { self.registers.h = self.next_byte(); },
            // LD L,n
            0x2E => { self.registers.l = self.next_byte(); },
            // LD A,B
            0x78 => { self.registers.a = self.registers.b; },
            // LD A,C
            0x79 => { self.registers.a = self.registers.c; },
            // LD A,D
            0x7A => { self.registers.a = self.registers.d; },
            // LD A,E
            0x7B => { self.registers.a = self.registers.e; },
            // LD A,H
            0x7C => { self.registers.a = self.registers.h; },
            // LD A,L
            0x7D => { self.registers.a = self.registers.l; },
            // LD A, (HL)
            0x7E => { self.registers.a = self.byte_at_hl(); },
            // LD A,A
            0x7F => { self.registers.a = self.registers.a },
            // LD B,B
            0x40 => { self.registers.b = self.registers.b; },
            // LD B,C
            0x41 => { self.registers.b = self.registers.c; },
            // LD B,D
            0x42 => { self.registers.b = self.registers.d; },
            // LD B,E
            0x43 => { self.registers.b = self.registers.e; },
            // LD B,H
            0x44 => { self.registers.b = self.registers.h; },
            // LD B,L
            0x45 => { self.registers.b = self.registers.l; },
            // LD B, (HL)
            0x46 => { self.registers.b = self.byte_at_hl(); },
            // LD B,A
            0x47 => { self.registers.b = self.registers.a; },
            // LD C,B
            0x48 => { self.registers.c = self.registers.b; },
            // LD C,C
            0x49 => { self.registers.c = self.registers.c; },
            // LD C,D
            0x4A => { self.registers.c = self.registers.d; },
            // LD C,E
            0x4B => { self.registers.c = self.registers.e; },
            // LD C,H
            0x4C => { self.registers.c = self.registers.h; },
            // LD C,L
            0x4D => { self.registers.c = self.registers.l; },
            // LD C,(HL)
            0x4E => { self.registers.c = self.byte_at_hl(); },
            // LD D,A
            0x4F => { self.registers.d = self.registers.a; },
            // LD D,B
            0x50 => { self.registers.d = self.registers.b; },
            // LD D,C
            0x51 => { self.registers.d = self.registers.c; },
            // LD D,D
            0x52 => { self.registers.d = self.registers.d; },
            // LD D,E
            0x53 => { self.registers.d = self.registers.e; },
            // LD D,H
            0x54 => { self.registers.d = self.registers.h; },
            // LD D,L
            0x55 => { self.registers.d = self.registers.l; },
            // LD D, (HL)
            0x56 => { self.registers.d = self.byte_at_hl(); },
            // LD D,A
            0x57 => { self.registers.d = self.registers.a; },
            // LD E,B
            0x58 => { self.registers.e = self.registers.b; },
            // LD E,C
            0x59 => { self.registers.e = self.registers.c; },
            // LD E,D
            0x5A => { self.registers.e = self.registers.d; },
            // LD E,E
            0x5B => { self.registers.e = self.registers.e; },
            // LD E,H
            0x5C => { self.registers.e = self.registers.h; },
            // LD E,L
            0x5D => { self.registers.e = self.registers.l; },
            // LD E, (HL)
            0x5E => { self.registers.e = self.byte_at_hl(); },
            // LD E,A
            0x5F => { self.registers.e = self.registers.a; },
            // LD H,B
            0x60 => { self.registers.h = self.registers.b; },
            // LD H,C
            0x61 => { self.registers.h = self.registers.c; },
            // LD H,D
            0x62 => { self.registers.h = self.registers.d; },
            // LD H,E
            0x63 => { self.registers.h = self.registers.e; },
            // LD H,H
            0x64 => { self.registers.h = self.registers.h; },
            // LD H,L
            0x65 => { self.registers.h = self.registers.l; },
            // LD H, (HL)
            0x66 => { self.registers.h = self.byte_at_hl(); },
            // LD H, A
            0x67 => { self.registers.h = self.registers.a; },
            // LD L,B
            0x68 => { self.registers.l = self.registers.b; },
            // LD L,C
            0x69 => { self.registers.l = self.registers.c; },
            // LD L,D
            0x6A => { self.registers.l = self.registers.d; },
            // LD L,E
            0x6B => { self.registers.l = self.registers.e; },
            // LD L,H
            0x6C => { self.registers.l = self.registers.h; },
            // LD L,L
            0x6D => { self.registers.l = self.registers.l; },
            // LD L, (HL)
            0x6E => { self.registers.l = self.byte_at_hl(); },
            // LD L,A
            0x6F => { self.registers.l = self.registers.a; }
            // LD (HL), B
            0x70 => { self.set_byte_at_hl(self.registers.b); },
            // LD (HL), C
            0x71 => { self.set_byte_at_hl(self.registers.c); },
            // LD (HL), D
            0x72 => { self.set_byte_at_hl(self.registers.d); },
            // LD (HL), E
            0x73 => { self.set_byte_at_hl(self.registers.e); },
            // LD (HL), H
            0x74 => { self.set_byte_at_hl(self.registers.h); },
            // LD (HL), L
            0x75 => { self.set_byte_at_hl(self.registers.l); },
            // LD (HL), n
            0x36 => { let n = self.next_byte(); self.set_byte_at_hl(n); },
            // LD A, (HL)
            0x0A => { self.registers.a = self.byte_at_hl(); },
            // LD A, (DE)
            0x1A => { self.registers.a = self.memory.read(self.registers.get_de()); },
            // LD A, (nn)
            0xFA => { let nn = self.next_u16(); self.registers.a = self.memory.read(nn); },
            // LD A, #
            0x3E  => { self.registers.a = self.next_byte(); },
            // LD (BC),A
            0x02 => { self.memory.write(self.registers.get_bc(), self.registers.a); },
            // LD (DE),A
            0x12 => { self.memory.write(self.registers.get_de(), self.registers.a); },
            // LD (HL),A
            0x77 => { self.memory.write(self.registers.get_hl(), self.registers.a); },
            // LD (nn),A
            0xEA => { let nn = self.next_u16(); self.memory.write(nn, self.registers.a); },
            // LD A, ($FF00 + C)
            0xF2 => { self.registers.a = self.memory.read(0xFF00) + self.registers.c; },
            // LD ($FF00+C),A
            0xE2 => { self.memory.write(0xFF00 + self.registers.c as u16, self.registers.a); },
            // LD A, (HL Dec/-)
            0x3A => { self.registers.a = self.byte_at_hl(); self.registers.set_hl(self.registers.get_hl() - 1); },
            // LD (HL Dec/-), A
            0x32 => { self.set_byte_at_hl(self.registers.a); self.registers.set_hl(self.registers.get_hl() - 1); },
            // LD A, (HL Inc/+)
            0x2A => { self.registers.a = self.byte_at_hl(); self.registers.set_hl(self.registers.get_hl() + 1); },
            // LD (HL Inc/+), A
            0x22 => { self.set_byte_at_hl(self.registers.a); self.registers.set_hl(self.registers.get_hl() + 1); },
            // LD ($FF00+n),A
            0xE0 => { let n = self.next_byte(); self.memory.write(0xFF00 + n as u16, self.registers.a); },
            // LD A, ($FF00+n)
            0xF0 => { let n = self.next_byte(); self.registers.a = self.memory.read(0xFF00 + n as u16)},
            // LD BC, nn
            0x01 => { let nn = self.next_u16(); self.registers.set_bc(nn); },
            // LD DE, nn
            0x11 => { let nn = self.next_u16(); self.registers.set_de(nn); },
            // LD HL, nn
            0x21 => { let nn = self.next_u16(); self.registers.set_hl(nn); },
            // LD SP, nn
            0x31 => { self.registers.sp = self.next_u16(); },
            // LD SP, HL
            0xF9 => { self.registers.sp = self.registers.get_hl(); },
            // LDHL SP, n
            0xF8 => {},
            // LD (nn), SP
            0x08 => {},
            // PUSH
            // POP
            // ADD
            // ADC
            // SUB
            // SBC
            // AND
            // OR
            // XOR
            // CP
            // INC
            // DEC
            // ADD (16 bit)
            // INC (16 bit)
            // DEC (16 bit)
            // SWAP
            // DAA
            // CPL
            // CCF
            // SCF
            // NOP
            0x00 => {},
            // HALT
            // STOP
            // DI
            0xF3 => {},
            // EI
            // RLCA
            // RLA
            // RRCA
            // RRA
            // RLC
            // RL
            // RRC
            // RR
            // SLA
            // SRA
            // SRL
            // BIT
            // SET
            // RES
            // JP
            // JP nn
            0xC3 => { let new_add = self.next_u16(); self.registers.pc = new_add; println!("jump to {:02X}", new_add); }
            // JR
            // CALL
            // RST
            // RET
            // RETI
            
            _ => panic!("Unknown Opcode: ${:02X} at address ${:04X}", opcode, self.registers.pc-1)
        }
    }

    fn byte_at_hl(&self) -> u8 {
        self.memory.read(self.registers.get_hl())
    }

    fn set_byte_at_hl(&mut self, value: u8) {
        self.memory.write(self.registers.get_hl(), value);
    }
}