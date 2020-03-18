mod memory;

use memory::Memory;
use memory::Registers;

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
        self.memory.print_cart_metadata();
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
            // PUSH AF
            0xF5 => { let af = self.registers.get_af(); self.memory.push_u16(&mut self.registers, af); },
            // PUSH BC
            0xC5 => { let bc = self.registers.get_bc(); self.memory.push_u16(&mut self.registers, bc); },
            // PUSH DE
            0xD5 => { let de = self.registers.get_de(); self.memory.push_u16(&mut self.registers, de); },
            // PUSH HL
            0xE5 => { let hl = self.registers.get_hl(); self.memory.push_u16(&mut self.registers, hl); },
            // POP AF
            0xF1 => { let nn = self.memory.pop_u16(&mut self.registers); self.registers.set_af(nn); },
            // POP BC
            0xC1 => { let nn = self.memory.pop_u16(&mut self.registers); self.registers.set_bc(nn); },
            // POP DE
            0xD1 => { let nn = self.memory.pop_u16(&mut self.registers); self.registers.set_de(nn); },
            // POP HL
            0xE1 => { let nn = self.memory.pop_u16(&mut self.registers); self.registers.set_hl(nn); },
            // ADD
            // ADC
            // SUB
            // SBC
            // AND
            // AND A
            0xA7 => { self.logical_and(self.registers.a); }
            // AND B
            0xA0 => { self.logical_and(self.registers.b); }
            // AND C
            0xA1 => { self.logical_and(self.registers.c); }
            // AND D
            0xA2 => { self.logical_and(self.registers.d); }
            // AND E
            0xA3 => { self.logical_and(self.registers.e); }
            // AND H
            0xA4 => { self.logical_and(self.registers.h); }
            // AND L
            0xA5 => { self.logical_and(self.registers.l); }
            // AND (HL)
            0xA6 => { self.logical_and(self.byte_at_hl()); }
            // AND n
            0xE6 => { let n = self.next_byte(); self.logical_and(n); }
            // OR
            // OR A
            0xB7 => { self.logical_or(self.registers.a); },
            // OR B
            0xB0 => { self.logical_or(self.registers.b); },
            // OR C
            0xB1 => { self.logical_or(self.registers.c); },
            // OR D
            0xB2 => { self.logical_or(self.registers.d); },
            // OR E
            0xB3 => { self.logical_or(self.registers.e); },
            // OR H
            0xB4 => { self.logical_or(self.registers.h); },
            // OR L
            0xB5 => { self.logical_or(self.registers.l); },
            // OR (HL)
            0xB6 => { self.logical_or(self.byte_at_hl()); },
            // OR n
            0xF6 => { let n = self.next_byte(); self.logical_or(n); },
            // XOR A
            0xAF => { self.logical_xor(self.registers.a); },
            // XOR B
            0xA8 => { self.logical_xor(self.registers.b); },
            // XOR C
            0xA9 => { self.logical_xor(self.registers.c); },
            // XOR D
            0xAA => { self.logical_xor(self.registers.d); },
            // XOR E
            0xAB => { self.logical_xor(self.registers.e); },
            // XOR H
            0xAC => { self.logical_xor(self.registers.h); },
            // XOR L
            0xAD => { self.logical_xor(self.registers.l); },
            // XOR (HL)
            0xAE => { self.logical_xor(self.byte_at_hl()); },
            // XOR n
            0xEE => { let n = self.next_byte(); self.logical_xor(n); },
            // CP
            // CP A
            0xBF => { self.compare(self.registers.a); },
            // CP B
            0xB8 => { self.compare(self.registers.b); },
            // CP C
            0xB9 => { self.compare(self.registers.c); },
            // CP D
            0xBA => {self.compare(self.registers.d); },
            // CP E
            0xBB => { self.compare(self.registers.e); },
            // CP H
            0xBC => { self.compare(self.registers.h); },
            // CP L
            0xBD => { self.compare(self.registers.l); },
            // CP (HL)
            0xBE => { self.compare(self.byte_at_hl()); },
            // CP n
            0xFE => { let n = self.next_byte(); self.compare(n); },
            // INC
            // INC A
            0x3C => { self.registers.a = self.add(self.registers.a, 1); },
            // INC B
            0x04 => { self.registers.b = self.add(self.registers.b, 1); },
            // INC C
            0x0C => { self.registers.c = self.add(self.registers.c, 1); },
            // INC D
            0x14 => { self.registers.d = self.add(self.registers.d, 1); },
            // INC E
            0x1C => { self.registers.e = self.add(self.registers.e, 1); },
            // INC H
            0x24 => { self.registers.h = self.add(self.registers.h, 1); },
            // INC L
            0x2C => { self.registers.l = self.add(self.registers.l, 1); },
            // INC (HL)
            0x34 => { 
                let new_hl = self.add(self.byte_at_hl(), 1); 
                self.set_byte_at_hl(new_hl);
            },
            // DEC
            // DEC A
            0x3D => { self.registers.a = self.subtract(self.registers.a, 1); },
            // DEC B
            0x05 => { self.registers.b = self.subtract(self.registers.b, 1); },
            // DEC C
            0x0D => { self.registers.c = self.subtract(self.registers.c, 1); },
            // DEC D
            0x15 => { self.registers.d = self.subtract(self.registers.d, 1); },
            // DEC E
            0x1D => { self.registers.e = self.subtract(self.registers.e, 1); },
            // DEC H
            0x25 => { self.registers.h = self.subtract(self.registers.h, 1); },
            // DEC L
            0x2D => { self.registers.l = self.subtract(self.registers.l, 1); },
            // DEC (HL)
            0x35 => { 
                let hl_value = self.subtract(self.byte_at_hl(), 1); 
                self.set_byte_at_hl(hl_value); 
            },
            // ADD (16 bit)
            // INC (16 bit)
            // INC BC
            0x03 => { self.registers.set_bc(self.registers.get_bc() + 1); },
            // INC DE
            0x13 => { self.registers.set_de(self.registers.get_de() + 1); },
            // INC HL
            0x23 => { self.registers.set_hl(self.registers.get_hl() + 1); },
            // INC SP
            0x33 => { self.registers.sp += 1; },
            // DEC (16 bit)
            // DEC BC
            0x0B => { self.registers.set_bc(self.registers.get_bc() - 1); },
            // DEC DE
            0x1B => { self.registers.set_de(self.registers.get_de() - 1); },
            // DEC HL
            0x2B => { self.registers.set_hl(self.registers.get_hl() - 1); },
            // DEC SP
            0x3B => { self.registers.sp -= 1; },
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
            0xC3 => { let new_add = self.next_u16(); self.registers.pc = new_add; println!("jump to {:02X}", new_add); },
            // JP (HL)
            0xE9 => { self.registers.pc = self.registers.get_hl(); },
            // JR n
            0x18 => { self.jump_by_n_if(true); },
            // JR NZ, *
            0x20 => { self.jump_by_n_if(!self.registers.zero_flag()); },
            // JR Z, *
            0x28 => { self.jump_by_n_if(self.registers.zero_flag()); },
            // JR NC, *
            0x30 => { self.jump_by_n_if(!self.registers.carry_flag()); },
            // JR C, *
            0x38 => { self.jump_by_n_if(self.registers.zero_flag()); },
            // CALL nn
            0xCD => { 
                let next_addr = self.next_u16(); 
                let next_instr = self.registers.pc;
                self.memory.push_u16(&mut self.registers, next_instr);
                self.registers.pc = next_addr;
            },
            // CALL NZ,nn
            0xC4 => { 
                if !self.registers.zero_flag() {
                    self.registers.pc = self.next_u16();
                }
            },
            // CALL Z,nn
            0xCC => { 
                if self.registers.zero_flag() {
                    self.registers.pc = self.next_u16();
                }
            },
            // CALL NC,nn
            0xD4 => { 
                if !self.registers.carry_flag() {
                    self.registers.pc = self.next_u16();
                }
            },
            // CALL C,nn
            0xDC => { 
                if self.registers.carry_flag() {
                    self.registers.pc = self.next_u16();
                }
            },
            // RST
            // RET
            0xC9 => {
                self.registers.pc = self.memory.pop_u16(&mut self.registers);
            }
            // RET NZ
            0xC0 => { self.return_if(!self.registers.zero_flag()); },
            // RET Z
            0xC8 => { self.return_if(self.registers.zero_flag()); },
            // RET NC
            0xD0 => { self.return_if(!self.registers.carry_flag()); },
            // RET C
            0xD8 => { self.return_if(self.registers.carry_flag()); },
            // RETI
            0xCB => {
                let cb_opcode = self.next_byte();
                match cb_opcode {
                    _ => panic!("Unknown Opcode after CB modifier: ${:02X} at address ${:04X}", cb_opcode, self.registers.pc-1)
                }
            }
            _ => panic!("Unknown Opcode: ${:02X} at address ${:04X}", opcode, self.registers.pc-1)
        }
    }
    /// param: `reg_val` - The value from a register from 
    /// which to logically or with register a
    fn logical_or(&mut self, reg_val: u8) {
        if self.registers.a | reg_val == 0 {
            self.registers.a = 0;
            self.registers.set_zero_flag();
        } else {
            self.registers.a = 1;
        }
        self.registers.reset_subtract_flag();
        self.registers.reset_half_carry_flag();
        self.registers.reset_carry_flag();
    }

    /// param: `reg_val` - The value from a register from 
    /// which to logically xor with register a
    fn logical_xor(&mut self, reg_val: u8) {
        if self.registers.a ^ reg_val == 0 {
            self.registers.a = 0;
            self.registers.set_zero_flag();
        } else {
            self.registers.a = 1;
        }
        self.registers.reset_subtract_flag();
        self.registers.reset_half_carry_flag();
        self.registers.reset_carry_flag();
    }

    /// param: `reg_val` - The value from a register from 
    /// which to logically and with register a
    fn logical_and(&mut self, reg_val: u8) {
        if self.registers.a & reg_val == 0 {
            self.registers.a = 0;
            self.registers.set_zero_flag();
        } else {
            self.registers.a = 1;
        }
        self.registers.set_half_carry_flag();
        self.registers.reset_subtract_flag();
        self.registers.reset_carry_flag();
    }

    /// If cond is true, jump to the current addres + n 
    /// where n is the immediately following signed byte
    fn jump_by_n_if(&mut self, cond: bool) {
        let n = self.next_byte();
        let next_addr = add_signed_u8_to_u16(self.registers.pc, n);
        if cond {
            self.registers.pc = next_addr;
            //println!("jumped to {:02X}", next_addr);
        }
    }

    /// Compare register A with n, A - n subtraction
    /// results are thrown away and flags are set
    fn compare(&mut self, n: u8) {
        let a = self.registers.a;
        if a == n {
            self.registers.set_zero_flag();
        } if a > n {
            self.registers.set_carry_flag();
        }
        if half_carry_subtraction(a, n) {
            self.registers.set_half_carry_flag();
        }
        self.registers.set_subtract_flag();
    }

    fn add(&mut self, first: u8, second: u8) -> u8 {
        if half_carry_addition(first, second) {
            self.registers.set_half_carry_flag();
        }
        self.registers.reset_subtract_flag();
        let new_val = first.wrapping_add(second);
        if new_val == 0 {
            self.registers.set_subtract_flag();
        }
        new_val
    }

    fn subtract(&mut self, first: u8, second: u8) -> u8 {
        if half_carry_subtraction(first, second) {
            self.registers.set_half_carry_flag();
        }
        self.registers.set_subtract_flag();
        let new_val = first.wrapping_sub(second);
        if new_val == 0 {
            self.registers.set_zero_flag();
        }
        new_val
    }

    // fn jump_if(&mut self, addr: u16, cond: bool) {
    //     if cond {
    //         self.registers.pc = addr;
    //         //println!("jumped to {:02X}", addr);
    //     }
    // }

    fn return_if(&mut self, cond: bool) {
        if cond {
            self.registers.pc = self.memory.pop_u16(&mut self.registers);
        }
    }

    /// Gets the value of the byte in memory at address stored in HL register
    fn byte_at_hl(&self) -> u8 {
        self.memory.read(self.registers.get_hl())
    }
    
    // Sets the value of the byte in memory at address stored in HL register
    fn set_byte_at_hl(&mut self, value: u8) {
        self.memory.write(self.registers.get_hl(), value);
    }
}

/// Converts u8 to i8 and adds to u16
fn add_signed_u8_to_u16(unsigned: u16, signed: u8) -> u16 {
    ((unsigned as i32) + i8::from_be_bytes([signed]) as i32) as u16
}

fn half_carry_addition(first: u8, second: u8) -> bool {
    (((first & 0x0F) + (second & 0x0F)) & 0x10) == 0x10
}

fn half_carry_addition_u16(first: u16, second: u16) -> bool {
    (((first & 0x00FF) + (second & 0x00FF)) & 0x0100) == 0x0100
}

fn half_carry_subtraction(first: u8, second: u8) -> bool {
    ((first & 0x0F) as i16 - (second & 0x0F) as i16) < 0
}

fn half_carry_subtraction_16(first: u8, second: u8) -> bool {
    ((first & 0x00FF) as i32 - (second & 0x00FF) as i32) < 0
}