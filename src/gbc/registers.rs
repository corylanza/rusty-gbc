const zero_flag_mask: u8 = 0b10000000;
const subtract_flag_mask: u8 = 0b01000000;
const half_carry_flag_mask: u8 = 0b00100000;
const carry_flag_mask: u8 = 0b00010000;

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
            sp: 0xFFFE,
            pc: 0x100
        }
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
        self.f & zero_flag_mask > 0
    }

    pub fn set_zero_flag(&mut self) {
        self.f |= zero_flag_mask;
    }

    pub fn reset_zero_flag(&mut self) {
        self.f &= !zero_flag_mask;
    }

    pub fn subtract_flag(&self) -> bool {
        self.f & subtract_flag_mask > 0
    }

    pub fn set_subtract_flag(&mut self) {
        self.f |= subtract_flag_mask;
    }

    pub fn reset_subtract_flag(&mut self) {
        self.f &= !subtract_flag_mask;
    }

    pub fn half_carry_flag(&self) -> bool {
        self.f & half_carry_flag_mask > 0
    }

    pub fn set_half_carry_flag(&mut self) {
        self.f = half_carry_flag_mask;
    }

    pub fn reset_half_carry_flag(&mut self) {
        self.f &= !half_carry_flag_mask;
    }

    pub fn carry_flag(&self) -> bool {
        self.f & carry_flag_mask > 0
    }

    pub fn set_carry_flag(&mut self) {
        self.f |= carry_flag_mask;
    }

    pub fn reset_carry_flag(&mut self) {
        self.f &= ! carry_flag_mask;
    }
}