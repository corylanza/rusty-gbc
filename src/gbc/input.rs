use super::JOYPAD_INTERRUPT;
const SELECT_BUTTON: u8 = 0b00100000;
const SELECT_DIRECTION: u8 = 0b00010000;

pub struct Input {
    joypad: u8,
    buttons: u8,
    pub interrupt: u8
}

impl Input {
    pub fn new() -> Self {
        Input {
            joypad: 0b00110000,
            buttons: 0xFF,
            interrupt: 0
        }
    }

    pub fn key_pressed(&mut self, key: Keycode) {
        self.interrupt = JOYPAD_INTERRUPT;
        self.buttons ^= key.bit();
    }

    pub fn key_released(&mut self, key: Keycode) {
        self.buttons ^= key.bit();
    }

    pub fn write_joypad(&mut self, value: u8) {
        self.joypad = value & 0b11110000;
    }

    pub fn read_joypad(&self) -> u8 {
        if self.joypad & SELECT_DIRECTION == 0 {
            self.joypad | ((0b11110000 & self.buttons) >> 4) | 0b11000000
        } else if self.joypad & SELECT_BUTTON == 0 {
            self.joypad | (0b00001111 & self.buttons) | 0b11000000
        } else {
            self.joypad | 0b11000000
        }
    }
}

pub enum Keycode {
    Start,
    Select,
    B,
    A,
    Down,
    Up,
    Left,
    Right,
}

impl Keycode {
    fn bit(&self) -> u8 {
        match self {
            Keycode::Start => 0b00001000,
            Keycode::Select => 0b00000100,
            Keycode::B => 0b00000010,
            Keycode::A => 0b00000001,
            Keycode::Down => 0b10000000,
            Keycode::Up => 0b01000000,
            Keycode::Left => 0b00100000,
            Keycode::Right => 0b00010000
        }
    }
}