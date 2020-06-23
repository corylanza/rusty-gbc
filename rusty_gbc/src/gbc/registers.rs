const ZERO_FLAG_MASK: u8 = 0b10000000;
const SUBTRACT_FLAG_MASK: u8 = 0b01000000;
const HALF_CARRY_FLAG_MASK: u8 = 0b00100000;
const CARRY_FLAG_MASK: u8 = 0b00010000;

pub struct Registers {
    pub a: u8,
	pub f: u8,
	pub b: u8,
	pub c: u8,
	pub d: u8,
	pub e: u8,
	pub h: u8,
	pub l: u8,
	pub sp: u16,
	pub pc: u16,
}

#[allow(dead_code)]
impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0
        }
    }

    pub fn get_af(&self) -> u16 {
        u16::from_be_bytes([self.a, self.f])
    }

    pub fn get_hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    pub fn get_bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    pub fn get_de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    pub fn set_af(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.a = bytes[0];
        self.f = bytes[1] & 0b11110000;
    }

    pub fn set_hl(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.h = bytes[0];
        self.l = bytes[1];
    }

    pub fn set_bc(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.b = bytes[0];
        self.c = bytes[1];
    }

    pub fn set_de(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.d = bytes[0];
        self.e = bytes[1];
    }

    pub fn zero_flag(&self) -> bool {
        self.f & ZERO_FLAG_MASK > 0
    }

    pub fn set_zero_flag(&mut self, to: bool) {
        self.f = if to { self.f | ZERO_FLAG_MASK } else { self.f & !ZERO_FLAG_MASK };
    }

    pub fn subtract_flag(&self) -> bool {
        self.f & SUBTRACT_FLAG_MASK > 0
    }

    pub fn set_subtract_flag(&mut self, to: bool) {
        self.f = if to { self.f | SUBTRACT_FLAG_MASK } else { self.f & !SUBTRACT_FLAG_MASK };
    }

    pub fn half_carry_flag(&self) -> bool {
        self.f & HALF_CARRY_FLAG_MASK > 0
    }

    pub fn set_half_carry_flag(&mut self, to: bool) {
        self.f = if to { self.f | HALF_CARRY_FLAG_MASK } else { self.f & !HALF_CARRY_FLAG_MASK };
    }

    pub fn carry_flag(&self) -> bool {
        self.f & CARRY_FLAG_MASK > 0
    }

    pub fn set_carry_flag(&mut self, to: bool) {
        self.f = if to { self.f | CARRY_FLAG_MASK } else { self.f & !CARRY_FLAG_MASK };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_flag () {
        let mut regs = Registers::new();
        // zero
        regs.set_zero_flag(true);
        assert_eq!(regs.zero_flag(), true);
        regs.set_zero_flag(false);
        assert_eq!(regs.zero_flag(), false);
        // half carry
        regs.set_half_carry_flag(true);
        assert_eq!(regs.half_carry_flag(), true);
        regs.set_half_carry_flag(false);
        assert_eq!(regs.half_carry_flag(), false);
        // carry
        regs.set_carry_flag(true);
        assert_eq!(regs.carry_flag(), true);
        regs.set_carry_flag(false);
        assert_eq!(regs.carry_flag(), false);
        // subtract
        regs.set_subtract_flag(true);
        assert_eq!(regs.subtract_flag(), true);
        regs.set_subtract_flag(false);
        assert_eq!(regs.subtract_flag(), false);
    }
}