mod memory;
mod boot;

use memory::Memory;
use memory::Registers;

pub struct Cpu {
    mem: Memory,
    regs: Registers,
    ime: bool // disables interrupts when false overriding IE register
}

impl Cpu {
    pub fn new(cartridge_path: &str) -> Cpu {
        Cpu {
            mem: Memory::new(cartridge_path),
            regs: Registers::new(),
            ime: true
        }
    }

    pub fn run(&mut self) {
        self.mem.print_cart_metadata();
        loop {
            self.cpu_step();
        }
    }

    pub fn next_byte(&mut self) -> u8 {
        let byte = self.mem.read(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        byte
    }

    pub fn next_u16(&mut self) -> u16 {
        u16::from_le_bytes([self.next_byte(), self.next_byte()])
    }

    /// returns number of cycles completed
    pub fn cpu_step(&mut self) -> u8 {
        let opcode = self.next_byte();
        //println!("executing ${:02X} at address ${:04X}", opcode, self.regs.pc-1);

        match opcode {
            // LD B,n
            0x06 => { self.regs.b = self.next_byte(); 8 },
            // LD C,n
            0x0E => { self.regs.c = self.next_byte(); 8 },
            // LD D,n
            0x16 => { self.regs.d = self.next_byte(); 8 },
            // LD E,n
            0x1E => { self.regs.e = self.next_byte(); 8 },
            // LD H,n
            0x26 => { self.regs.h = self.next_byte(); 8 },
            // LD L,n
            0x2E => { self.regs.l = self.next_byte(); 8 },
            // LD A,B
            0x78 => { self.regs.a = self.regs.b; 4 },
            // LD A,C
            0x79 => { self.regs.a = self.regs.c; 4 },
            // LD A,D
            0x7A => { self.regs.a = self.regs.d; 4 },
            // LD A,E
            0x7B => { self.regs.a = self.regs.e; 4 },
            // LD A,H
            0x7C => { self.regs.a = self.regs.h; 4 },
            // LD A,L
            0x7D => { self.regs.a = self.regs.l; 4 },
            // LD A, (HL)
            0x7E => { self.regs.a = self.byte_at_hl(); 8 },
            // LD A,A
            0x7F => { self.regs.a = self.regs.a; 4 },
            // LD B,B
            0x40 => { self.regs.b = self.regs.b; 4 },
            // LD B,C
            0x41 => { self.regs.b = self.regs.c; 4 },
            // LD B,D
            0x42 => { self.regs.b = self.regs.d; 4 },
            // LD B,E
            0x43 => { self.regs.b = self.regs.e; 4 },
            // LD B,H
            0x44 => { self.regs.b = self.regs.h; 4 },
            // LD B,L
            0x45 => { self.regs.b = self.regs.l; 4 },
            // LD B, (HL)
            0x46 => { self.regs.b = self.byte_at_hl(); 8 },
            // LD B,A
            0x47 => { self.regs.b = self.regs.a; 4 },
            // LD C,B
            0x48 => { self.regs.c = self.regs.b; 4 },
            // LD C,C
            0x49 => { self.regs.c = self.regs.c; 4 },
            // LD C,D
            0x4A => { self.regs.c = self.regs.d; 4 },
            // LD C,E
            0x4B => { self.regs.c = self.regs.e; 4},
            // LD C,H
            0x4C => { self.regs.c = self.regs.h; 4 },
            // LD C,L
            0x4D => { self.regs.c = self.regs.l; 4 },
            // LD C,(HL)
            0x4E => { self.regs.c = self.byte_at_hl(); 8 },
            // LD D,A
            0x4F => { self.regs.d = self.regs.a; 4 },
            // LD D,B
            0x50 => { self.regs.d = self.regs.b; 4 },
            // LD D,C
            0x51 => { self.regs.d = self.regs.c; 4 },
            // LD D,D
            0x52 => { self.regs.d = self.regs.d; 4 },
            // LD D,E
            0x53 => { self.regs.d = self.regs.e; 4 },
            // LD D,H
            0x54 => { self.regs.d = self.regs.h; 4 },
            // LD D,L
            0x55 => { self.regs.d = self.regs.l; 4 },
            // LD D, (HL)
            0x56 => { self.regs.d = self.byte_at_hl(); 8 },
            // LD D,A
            0x57 => { self.regs.d = self.regs.a; 4 },
            // LD E,B
            0x58 => { self.regs.e = self.regs.b; 4 },
            // LD E,C
            0x59 => { self.regs.e = self.regs.c; 4 },
            // LD E,D
            0x5A => { self.regs.e = self.regs.d; 4 },
            // LD E,E
            0x5B => { self.regs.e = self.regs.e; 4 },
            // LD E,H
            0x5C => { self.regs.e = self.regs.h; 4 },
            // LD E,L
            0x5D => { self.regs.e = self.regs.l; 4 },
            // LD E, (HL)
            0x5E => { self.regs.e = self.byte_at_hl(); 8 },
            // LD E,A
            0x5F => { self.regs.e = self.regs.a; 4 },
            // LD H,B
            0x60 => { self.regs.h = self.regs.b; 4 },
            // LD H,C
            0x61 => { self.regs.h = self.regs.c; 4 },
            // LD H,D
            0x62 => { self.regs.h = self.regs.d; 4 },
            // LD H,E
            0x63 => { self.regs.h = self.regs.e; 4 },
            // LD H,H
            0x64 => { self.regs.h = self.regs.h; 4 },
            // LD H,L
            0x65 => { self.regs.h = self.regs.l; 4 },
            // LD H, (HL)
            0x66 => { self.regs.h = self.byte_at_hl(); 8 },
            // LD H, A
            0x67 => { self.regs.h = self.regs.a; 4 },
            // LD L,B
            0x68 => { self.regs.l = self.regs.b; 4 },
            // LD L,C
            0x69 => { self.regs.l = self.regs.c; 4 },
            // LD L,D
            0x6A => { self.regs.l = self.regs.d; 4 },
            // LD L,E
            0x6B => { self.regs.l = self.regs.e; 4 },
            // LD L,H
            0x6C => { self.regs.l = self.regs.h; 4 },
            // LD L,L
            0x6D => { self.regs.l = self.regs.l; 4 },
            // LD L, (HL)
            0x6E => { self.regs.l = self.byte_at_hl(); 8 },
            // LD L,A
            0x6F => { self.regs.l = self.regs.a; 4 }
            // LD (HL), B
            0x70 => { self.set_byte_at_hl(self.regs.b); 8 },
            // LD (HL), C
            0x71 => { self.set_byte_at_hl(self.regs.c); 8 },
            // LD (HL), D
            0x72 => { self.set_byte_at_hl(self.regs.d); 8 },
            // LD (HL), E
            0x73 => { self.set_byte_at_hl(self.regs.e); 8 },
            // LD (HL), H
            0x74 => { self.set_byte_at_hl(self.regs.h); 8 },
            // LD (HL), L
            0x75 => { self.set_byte_at_hl(self.regs.l); 8 },
            // LD (HL), n
            0x36 => { let n = self.next_byte(); self.set_byte_at_hl(n); 12 },
            // LD A, (BC)
            0x0A => { self.regs.a = self.mem.read(self.regs.get_bc()); 8 },
            // LD A, (DE)
            0x1A => { self.regs.a = self.mem.read(self.regs.get_de()); 8 },
            // LD A, (nn)
            0xFA => { let nn = self.next_u16(); self.regs.a = self.mem.read(nn); 16 },
            // LD A, #
            0x3E  => { self.regs.a = self.next_byte(); 8},
            // LD (BC),A
            0x02 => { self.mem.write(self.regs.get_bc(), self.regs.a); 8 },
            // LD (DE),A
            0x12 => { self.mem.write(self.regs.get_de(), self.regs.a); 8 },
            // LD (HL),A
            0x77 => { self.mem.write(self.regs.get_hl(), self.regs.a); 8 },
            // LD (nn),A
            0xEA => { let nn = self.next_u16(); self.mem.write(nn, self.regs.a); 8 },
            // LD A, ($FF00 + C)
            0xF2 => { self.regs.a = self.mem.read(0xFF00) + self.regs.c; 8 },
            // LD ($FF00+C),A
            0xE2 => { self.mem.write(0xFF00 + self.regs.c as u16, self.regs.a); 8 },
            // LD A, (HL Dec/-)
            0x3A => { self.regs.a = self.byte_at_hl(); self.regs.set_hl(self.regs.get_hl() - 1); 8 },
            // LD (HL Dec/-), A
            0x32 => { self.set_byte_at_hl(self.regs.a); self.regs.set_hl(self.regs.get_hl() - 1); 8 },
            // LD A, (HL Inc/+)
            0x2A => { self.regs.a = self.byte_at_hl(); self.regs.set_hl(self.regs.get_hl() + 1); 8 },
            // LD (HL Inc/+), A
            0x22 => { self.set_byte_at_hl(self.regs.a); self.regs.set_hl(self.regs.get_hl() + 1); 8 },
            // LD ($FF00+n),A
            0xE0 => { let n = self.next_byte(); self.mem.write(0xFF00 + n as u16, self.regs.a); 12 },
            // LD A, ($FF00+n)
            0xF0 => { let n = self.next_byte(); self.regs.a = self.mem.read(0xFF00 + n as u16); 12 },
            // LD BC, nn
            0x01 => { let nn = self.next_u16(); self.regs.set_bc(nn); 12 },
            // LD DE, nn
            0x11 => { let nn = self.next_u16(); self.regs.set_de(nn); 12 },
            // LD HL, nn
            0x21 => { let nn = self.next_u16(); self.regs.set_hl(nn); 12 },
            // LD SP, nn
            0x31 => { self.regs.sp = self.next_u16(); 12 },
            // LD SP, HL
            0xF9 => { self.regs.sp = self.regs.get_hl(); 8 },
            // LDHL SP, n
            0xF8 => { 
                panic!("nooooo");
                //let n = self.next_byte(); 
                //self.registers.set_hl(self.memory.read(n));
                //12
            },
            // LD (nn), SP
            0x08 => { self.regs.sp = self.next_u16(); 20 },
            // PUSH AF
            0xF5 => { let af = self.regs.get_af(); self.mem.push_u16(&mut self.regs, af); 16 },
            // PUSH BC
            0xC5 => { let bc = self.regs.get_bc(); self.mem.push_u16(&mut self.regs, bc); 16 },
            // PUSH DE
            0xD5 => { let de = self.regs.get_de(); self.mem.push_u16(&mut self.regs, de); 16 },
            // PUSH HL
            0xE5 => { let hl = self.regs.get_hl(); self.mem.push_u16(&mut self.regs, hl); 16 },
            // POP AF
            0xF1 => { let nn = self.mem.pop_u16(&mut self.regs); self.regs.set_af(nn); 12 },
            // POP BC
            0xC1 => { let nn = self.mem.pop_u16(&mut self.regs); self.regs.set_bc(nn); 12 },
            // POP DE
            0xD1 => { let nn = self.mem.pop_u16(&mut self.regs); self.regs.set_de(nn); 12 },
            // POP HL
            0xE1 => { let nn = self.mem.pop_u16(&mut self.regs); self.regs.set_hl(nn); 12 },
            // ADD A,A
            0x87 => { self.regs.a = self.add(self.regs.a, self.regs.a); 4 },
            // ADD A,B
            0x80 => { self.regs.a = self.add(self.regs.a, self.regs.b); 4 },
            // ADD A,C
            0x81 => { self.regs.a = self.add(self.regs.a, self.regs.c); 4 },
            // ADD A,D
            0x82 => { self.regs.a = self.add(self.regs.a, self.regs.d); 4 },
            // ADD A,E
            0x83 => { self.regs.a = self.add(self.regs.a, self.regs.e); 4 },
            // ADD A,H
            0x84 => { self.regs.a = self.add(self.regs.a, self.regs.h); 4 },
            // ADd A,L
            0x85 => { self.regs.a = self.add(self.regs.a, self.regs.l); 4 },
            // ADd A, (HL)
            0x86 => { self.regs.a = self.add(self.regs.a, self.byte_at_hl()); 8 },
            // ADD A,n
            0xC6 => { let n = self.next_byte(); self.regs.a = self.add(self.regs.a, n); 8 },
            // ADC A,A
            0x8F => { self.regs.a = self.add_carry(self.regs.a, self.regs.a); 4 },
            // ADC A,B
            0x88 => { self.regs.a = self.add(self.regs.a, self.regs.b); 4 },
            // ADC A,C
            0x89 => { self.regs.a = self.add(self.regs.a, self.regs.c); 4 },
            // ADC A,D
            0x8A => { self.regs.a = self.add(self.regs.a, self.regs.d); 4 },
            // ADC A,E
            0x8B => { self.regs.a = self.add(self.regs.a, self.regs.e); 4 },
            // ADC A,H
            0x8C => { self.regs.a = self.add(self.regs.a, self.regs.h); 4 },
            // ADC A,L
            0x8D => { self.regs.a = self.add(self.regs.a, self.regs.l); 4 },
            // ADC A, (HL)
            0x8E => { self.regs.a = self.add(self.regs.a, self.byte_at_hl()); 8 },
            // ADC A,n
            0xCE => { let n = self.next_byte(); self.regs.a = self.add(self.regs.a, n); 8 },
            // SUB A
            0x97 => { self.regs.a = self.subtract(self.regs.a, self.regs.a); 4 },
            // SUB B
            0x90 => { self.regs.b = self.subtract(self.regs.a, self.regs.b); 4 },
            // SUB C
            0x91 => { self.regs.c = self.subtract(self.regs.a, self.regs.c); 4 },
            // SUB D
            0x92 => { self.regs.d = self.subtract(self.regs.a, self.regs.d); 4 },
            // SUB E
            0x93 => { self.regs.e = self.subtract(self.regs.a, self.regs.e); 4 },
            // SUB H
            0x94 => { self.regs.h = self.subtract(self.regs.a, self.regs.h); 4 },
            // SUB L
            0x95 => { self.regs.l = self.subtract(self.regs.a, self.regs.l); 4 },
            // SUB (HL)
            0x96 => { self.regs.a = self.subtract(self.regs.a, self.byte_at_hl()); 8 },
            // SUB n
            0xD6 => { 
                let n = self.next_byte(); 
                self.regs.a = self.subtract(self.regs.a, n);
                8
            },
            // SBC
            // AND
            // AND A
            0xA7 => { self.logical_and(self.regs.a); 4 }
            // AND B
            0xA0 => { self.logical_and(self.regs.b); 4 }
            // AND C
            0xA1 => { self.logical_and(self.regs.c); 4 }
            // AND D
            0xA2 => { self.logical_and(self.regs.d); 4 }
            // AND E
            0xA3 => { self.logical_and(self.regs.e); 4 }
            // AND H
            0xA4 => { self.logical_and(self.regs.h); 4 }
            // AND L
            0xA5 => { self.logical_and(self.regs.l); 4 }
            // AND (HL)
            0xA6 => { self.logical_and(self.byte_at_hl()); 8 }
            // AND n
            0xE6 => { let n = self.next_byte(); self.logical_and(n); 8 }
            // OR
            // OR A
            0xB7 => { self.logical_or(self.regs.a); 4 },
            // OR B
            0xB0 => { self.logical_or(self.regs.b); 4 },
            // OR C
            0xB1 => { self.logical_or(self.regs.c); 4 },
            // OR D
            0xB2 => { self.logical_or(self.regs.d); 4 },
            // OR E
            0xB3 => { self.logical_or(self.regs.e); 4 },
            // OR H
            0xB4 => { self.logical_or(self.regs.h); 4 },
            // OR L
            0xB5 => { self.logical_or(self.regs.l); 4 },
            // OR (HL)
            0xB6 => { self.logical_or(self.byte_at_hl()); 8 },
            // OR n
            0xF6 => { let n = self.next_byte(); self.logical_or(n); 8 },
            // XOR A
            0xAF => { self.logical_xor(self.regs.a); 4 },
            // XOR B
            0xA8 => { self.logical_xor(self.regs.b); 4 },
            // XOR C
            0xA9 => { self.logical_xor(self.regs.c); 4 },
            // XOR D
            0xAA => { self.logical_xor(self.regs.d); 4 },
            // XOR E
            0xAB => { self.logical_xor(self.regs.e); 4 },
            // XOR H
            0xAC => { self.logical_xor(self.regs.h); 4 },
            // XOR L
            0xAD => { self.logical_xor(self.regs.l); 4 },
            // XOR (HL)
            0xAE => { self.logical_xor(self.byte_at_hl()); 8},
            // XOR n
            0xEE => { let n = self.next_byte(); self.logical_xor(n); 8 },
            // CP
            // CP A
            0xBF => { self.compare(self.regs.a); 4 },
            // CP B
            0xB8 => { self.compare(self.regs.b); 4 },
            // CP C
            0xB9 => { self.compare(self.regs.c); 4 },
            // CP D
            0xBA => {self.compare(self.regs.d); 4 },
            // CP E
            0xBB => { self.compare(self.regs.e); 4 },
            // CP H
            0xBC => { self.compare(self.regs.h); 4 },
            // CP L
            0xBD => { self.compare(self.regs.l); 4 },
            // CP (HL)
            0xBE => { self.compare(self.byte_at_hl()); 8},
            // CP n
            0xFE => { let n = self.next_byte(); self.compare(n); 8 },
            // INC
            // INC A
            0x3C => { self.regs.a = self.add(self.regs.a, 1); 4 },
            // INC B
            0x04 => { self.regs.b = self.add(self.regs.b, 1); 4 },
            // INC C
            0x0C => { self.regs.c = self.add(self.regs.c, 1); 4 },
            // INC D
            0x14 => { self.regs.d = self.add(self.regs.d, 1); 4 },
            // INC E
            0x1C => { self.regs.e = self.add(self.regs.e, 1); 4 },
            // INC H
            0x24 => { self.regs.h = self.add(self.regs.h, 1); 4 },
            // INC L
            0x2C => { self.regs.l = self.add(self.regs.l, 1); 4 },
            // INC (HL)
            0x34 => { 
                let new_hl = self.add(self.byte_at_hl(), 1); 
                self.set_byte_at_hl(new_hl);
                12
            },
            // DEC
            // DEC A
            0x3D => { self.regs.a = self.subtract(self.regs.a, 1); 4 },
            // DEC B
            0x05 => { self.regs.b = self.subtract(self.regs.b, 1); 4 },
            // DEC C
            0x0D => { self.regs.c = self.subtract(self.regs.c, 1); 4 },
            // DEC D
            0x15 => { self.regs.d = self.subtract(self.regs.d, 1); 4 },
            // DEC E
            0x1D => { self.regs.e = self.subtract(self.regs.e, 1); 4 },
            // DEC H
            0x25 => { self.regs.h = self.subtract(self.regs.h, 1); 4 },
            // DEC L
            0x2D => { self.regs.l = self.subtract(self.regs.l, 1); 4 },
            // DEC (HL)
            0x35 => { 
                let new_hl = self.subtract(self.byte_at_hl(), 1); 
                self.set_byte_at_hl(new_hl); 
                12
            },
            // ADD (16 bit)
            // ADD HL,BC
            0x09 => { 
                let hl = self.add_u16(self.regs.get_hl(), self.regs.get_bc());
                self.regs.set_hl(hl);
                8
            },
            // ADD HL,DE
            0x19 => { 
                let hl = self.add_u16(self.regs.get_hl(), self.regs.get_de());
                self.regs.set_hl(hl);
                8
            },
            // ADD HL,HL
            0x29 => { 
                let hl = self.add_u16(self.regs.get_hl(), self.regs.get_hl());
                self.regs.set_hl(hl);
                8
            },
            // ADD HL,SP
            0x39 => { 
                let hl = self.add_u16(self.regs.get_hl(), self.regs.sp);
                self.regs.set_hl(hl); 
                8
            },
            // ADD SP,n
            // INC (16 bit)
            // INC BC
            0x03 => { self.regs.set_bc(self.regs.get_bc() + 1); 8 },
            // INC DE
            0x13 => { self.regs.set_de(self.regs.get_de() + 1); 8 },
            // INC HL
            0x23 => { self.regs.set_hl(self.regs.get_hl() + 1); 8 },
            // INC SP
            0x33 => { self.regs.sp += 1; 8 },
            // DEC (16 bit)
            // DEC BC
            0x0B => { self.regs.set_bc(self.regs.get_bc() - 1); 8 },
            // DEC DE
            0x1B => { self.regs.set_de(self.regs.get_de() - 1); 8 },
            // DEC HL
            0x2B => { self.regs.set_hl(self.regs.get_hl() - 1); 8 },
            // DEC SP
            0x3B => { self.regs.sp -= 1; 8 },
            // DAA
            // CPL
            0x2F => {
                self.regs.a = !self.regs.a; 
                self.regs.set_subtract_flag(true); 
                self.regs.set_half_carry_flag(true);
                4
            },
            // CCF
            0x3f => {
                self.regs.set_carry_flag(!self.regs.carry_flag());
                self.regs.set_subtract_flag(false);
                self.regs.set_half_carry_flag(false);
                4
            }
            // SCF
            0x37 => { 
                self.regs.set_subtract_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.set_carry_flag(true);
                4
            },
            // NOP
            0x00 => { 4 },
            // HALT
            // STOP
            // DI
            0xF3 => { self.ime = false; 4 },
            // EI
            0xFB => { self.ime = true; 4 }
            // RLCA
            0x07 => { 
                self.regs.set_carry_flag(self.regs.a & 0b10000000 > 0);
                self.regs.set_subtract_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.a <<= 1;
                self.regs.set_zero_flag(self.regs.a == 0);
                4
            },
            // RLA
            0x17 => { 
                self.regs.set_carry_flag(self.regs.a & 0b10000000 > 0);
                self.regs.set_subtract_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.a = (self.regs.a << 1) | if self.regs.carry_flag() {0b00000001} else {0} ;
                self.regs.set_zero_flag(self.regs.a == 0);
                4
            },
            // RRCA
            0x0F => {
                self.regs.set_carry_flag(self.regs.a & 0b00000001 > 0);
                self.regs.set_subtract_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.a  >>= 1;
                self.regs.set_zero_flag(self.regs.a == 0);
                4
            },
            // RRA
            0x1F => {
                self.regs.set_carry_flag(self.regs.a & 0b00000001 > 0);
                self.regs.set_subtract_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.a  = (self.regs.a >> 1) | if self.regs.carry_flag() {0b10000000} else {0};
                self.regs.set_zero_flag(self.regs.a == 0);
                4
            },
            // JP
            // JP nn
            0xC3 => { self.jump_to_nn_if(true); 12 },
            // JP NZ
            0xC2 => { self.jump_to_nn_if(!self.regs.zero_flag()); 12 },
            // JP Z
            0xCA => { self.jump_to_nn_if(self.regs.zero_flag()); 12 },
            // JP NC
            0xD2 => { self.jump_to_nn_if(!self.regs.carry_flag()); 12 },
            // JP C
            0xDA => { self.jump_to_nn_if(self.regs.carry_flag()); 12 },
            // JP HL
            0xE9 => { self.regs.pc = self.regs.get_hl(); 4 },
            // JR n
            0x18 => { self.jump_by_n_if(true); 8 },
            // JR NZ, *
            0x20 => { self.jump_by_n_if(!self.regs.zero_flag()); 8 },
            // JR Z, *
            0x28 => { self.jump_by_n_if(self.regs.zero_flag()); 8 },
            // JR NC, *
            0x30 => { self.jump_by_n_if(!self.regs.carry_flag()); 8 },
            // JR C, *
            0x38 => { self.jump_by_n_if(self.regs.carry_flag()); 8 },
            // CALL nn
            0xCD => { self.call_if(true); 12 },
            // CALL NZ,nn
            0xC4 => { self.call_if(!self.regs.zero_flag()); 12 },
            // CALL Z,nn
            0xCC => { self.call_if(self.regs.zero_flag()); 12 },
            // CALL NC,nn
            0xD4 => { self.call_if(!self.regs.carry_flag()); 12 },
            // CALL C,nn
            0xDC => { self.call_if(self.regs.carry_flag()); 12 },
            // RST 0x00
            0xC7 => { self.restart(0x00); 32 },
            // RST 0x08
            0xCF => { self.restart(0x08); 32 },
            // RST 0x10
            0xD7 => { self.restart(0x10); 32 },
            // RST 0x18
            0xDF => { self.restart(0x18); 32 },
            // RST 0x20
            0xE7 => { self.restart(0x20); 32 },
            // RST 0x28
            0xEF => { self.restart(0x28); 32 },
            // RST 0x30
            0xF7 => { self.restart(0x30); 32 },
            // RST 0x38
            0xFF => { self.restart(0x38); 32 },
            // RET
            0xC9 => { self.return_if(true); 8 }
            // RET NZ
            0xC0 => { self.return_if(!self.regs.zero_flag()); 8 },
            // RET Z
            0xC8 => { self.return_if(self.regs.zero_flag()); 8 },
            // RET NC
            0xD0 => { self.return_if(!self.regs.carry_flag()); 8 },
            // RET C
            0xD8 => { self.return_if(self.regs.carry_flag()); 8 },
            // RETI
            // CB ops
            0xCB => { self.cb_opcode_step() },
            _ => panic!("Unknown Opcode: ${:02X} at address ${:04X}", opcode, self.regs.pc-1)
        }
    }
    /// param: `reg_val` - The value from a register from 
    /// which to logically or with register a
    fn logical_or(&mut self, reg_val: u8) {
        self.regs.a = if self.regs.a > 0 || reg_val > 0 { 1 } else { 0 };
        self.regs.set_zero_flag(self.regs.a == 0);
        self.regs.set_subtract_flag(false);
        self.regs.set_half_carry_flag(false);
        self.regs.set_carry_flag(false);
    }

    /// param: `reg_val` - The value from a register from 
    /// which to logically xor with register a
    fn logical_xor(&mut self, reg_val: u8) {
        self.regs.a = if (self.regs.a > 0 && reg_val == 0) || (self.regs.a == 0 && reg_val > 0) { 1 } else { 0 };
        self.regs.set_zero_flag(self.regs.a == 0);
        self.regs.set_subtract_flag(false);
        self.regs.set_half_carry_flag(false);
        self.regs.set_carry_flag(false);
    }

    /// param: `reg_val` - The value from a register from 
    /// which to logically and with register a
    fn logical_and(&mut self, reg_val: u8) {
        self.regs.a = if self.regs.a > 0 && reg_val > 0 { 1 } else { 0 };
        self.regs.set_zero_flag(self.regs.a == 0);
        self.regs.set_half_carry_flag(true);
        self.regs.set_subtract_flag(false);
        self.regs.set_carry_flag(false);
    }

    /// If cond is true, jump to the current addres + n 
    /// where n is the immediately following signed byte
    fn jump_by_n_if(&mut self, cond: bool) {
        let n = self.next_byte();
        let next_addr = add_signed_u8_to_u16(self.regs.pc, n);
        if cond {
            self.regs.pc = next_addr;
        }
    }

    fn jump_to_nn_if(&mut self, cond: bool) {
        let nn = self.next_u16();
        if cond {
            self.regs.pc = nn;
        }
    }

    /// jump to the current 0x0000 + n, push current address to stack
    fn restart(&mut self, n: u8) {
        let next_addr = self.regs.pc;
        self.mem.push_u16(&mut self.regs, next_addr);
        self.regs.pc = n as u16;
    }

    /// Compare register A with n, A - n subtraction
    /// results are thrown away and flags are set
    fn compare(&mut self, n: u8) {
        let a = self.regs.a;
        self.regs.set_zero_flag(a == n);
        self.regs.set_carry_flag(a < n);
        self.regs.set_half_carry_flag(half_carry_subtraction(a, n));
        self.regs.set_subtract_flag(true);
    }

    fn add(&mut self, first: u8, second: u8) -> u8 {
        self.regs.set_half_carry_flag(half_carry_addition(first, second));
        self.regs.set_carry_flag((first as u16) + (second as u16) > 0xFF);
        self.regs.set_subtract_flag(false);
        let new_val = first.wrapping_add(second);
        self.regs.set_zero_flag(new_val == 0);
        new_val
    }

    fn add_carry(&mut self, first: u8, second: u8) -> u8 {
        let second = second + if self.regs.carry_flag() { 1 } else { 0 };
        self.regs.set_half_carry_flag(half_carry_addition(first, second));
        self.regs.set_carry_flag((first as u16) + (second as u16) > 0xFF);
        self.regs.set_subtract_flag(false);
        let new_val = first.wrapping_add(second);
        self.regs.set_zero_flag(new_val == 0);
        new_val
    }

    fn add_u16(&mut self, first: u16, second: u16) -> u16 {
        self.regs.set_half_carry_flag(half_carry_addition_u16(first, second));
        self.regs.set_carry_flag((first as u32) + (second as u32) > 0xFFFF);
        self.regs.set_subtract_flag(false);
        let new_val = first.wrapping_add(second);
        self.regs.set_zero_flag(new_val == 0);
        new_val
    }

    fn subtract(&mut self, first: u8, second: u8) -> u8 {
        self.regs.set_half_carry_flag(half_carry_subtraction(first, second));
        self.regs.set_carry_flag((first as i16) - (second as i16) > 0x00);
        self.regs.set_subtract_flag(true);
        let new_val = first.wrapping_sub(second);
        self.regs.set_zero_flag(new_val == 0);
        new_val
    }

    fn call_if(&mut self, cond: bool) {
        let next_addr = self.next_u16(); 
        if cond {
            let next_instr = self.regs.pc;
            self.mem.push_u16(&mut self.regs, next_instr);
            self.regs.pc = next_addr;
        }
    }

    fn return_if(&mut self, cond: bool) {
        if cond {
            self.regs.pc = self.mem.pop_u16(&mut self.regs);
        }
    }

    fn rotate_right(&mut self, value: u8) -> u8 {
        self.regs.set_carry_flag(value & 0b00000001 > 0);
        self.regs.set_subtract_flag(false);
        self.regs.set_half_carry_flag(false);
        let new_value = (value >> 1) | if self.regs.carry_flag() {0b10000000} else {0};
        self.regs.set_zero_flag(new_value == 0);
        new_value
    }

    /// Gets the value of the byte in memory at address stored in HL register
    fn byte_at_hl(&self) -> u8 {
        self.mem.read(self.regs.get_hl())
    }
    
    // Sets the value of the byte in memory at address stored in HL register
    fn set_byte_at_hl(&mut self, value: u8) {
        self.mem.write(self.regs.get_hl(), value);
    }

    fn cb_opcode_step(&mut self) -> u8 {
        let cb_opcode = self.next_byte();
        match cb_opcode {
            // RLCA
            // RLA
            // RRCA
            // RRA
            // RLC
            // RL D
            0x12 => {
                self.regs.set_carry_flag(self.regs.d & 0b10000000 > 0);
                self.regs.set_subtract_flag(false);
                self.regs.set_half_carry_flag(false);
                self.regs.d = self.regs.d << 1;
                self.regs.set_zero_flag(self.regs.a == 0);
                8
            },
            // RR
            // RR C
                0x19 => { self.regs.c = self.rotate_right(self.regs.c); 8 },
            // // RR D
                0x1A => { self.regs.d = self.rotate_right(self.regs.d); 8 },
            // // RR E
                0x1B => { self.regs.e = self.rotate_right(self.regs.e); 8 },
            // SLA
            // SRA
            // SRL
            // BIT
            // BIT 1,D
            0x42 => { 
                let t = self.regs.d & (1 << 0);
                self.regs.set_zero_flag(t & 0xFF == 0);
                self.regs.set_subtract_flag(false);
                self.regs.set_half_carry_flag(true);
                8
            },
            // BIT b,D
            0x52 => { 
                let t = self.regs.d & (2 << 0);
                self.regs.set_zero_flag(t & 0xFF == 0);
                self.regs.set_subtract_flag(false);
                self.regs.set_half_carry_flag(true);
                8
            },
            _ => panic!("Unknown Opcode after CB modifier: ${:02X} at address ${:04X}", cb_opcode, self.regs.pc-1)
        }
    }
}

/// Converts u8 to i8 and adds to u16
fn add_signed_u8_to_u16(unsigned: u16, signed: u8) -> u16 {
    ((unsigned as i32) + i8::from_be_bytes([signed]) as i32) as u16
}

fn half_carry_addition(first: u8, second: u8) -> bool {
    (((first & 0x0F) + (second & 0x0F)) & 0x10) == 0x10
}

fn half_carry_subtraction(first: u8, second: u8) -> bool {
    ((first & 0x0F) as i16 - (second & 0x0F) as i16) < 0
}

fn half_carry_addition_u16(first: u16, second: u16) -> bool {
    (((first & 0x00FF) + (second & 0x00FF)) & 0x0100) == 0x0100
}

// fn half_carry_subtraction_16(first: u8, second: u8) -> bool {
//     ((first & 0x00FF) as i32 - (second & 0x00FF) as i32) < 0
// }