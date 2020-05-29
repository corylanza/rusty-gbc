use super::TIMER_INTERRUPT;
const TIMER_ENABLE:u8 = 0b00000100;
const TIMER_CLOCK_SPEED:u8 = 0b00000011;

pub struct Timer {
    pub div: u8,
    /// Timer counter
    pub tima: u8,
    // Timer modulo
    pub tma: u8,
    enabled: bool,
    increment_in_cpu_cycles: u16,
    counter: u16,
    pub interrupt: u8
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            enabled: false,
            increment_in_cpu_cycles: 1024,
            counter: 0,
            interrupt: 0,
        }
    }

    pub fn timer_step(&mut self, cycles: u8) {
        self.div = self.div.wrapping_add(cycles);
        if self.enabled {
            self.counter += cycles as u16;
            if self.counter > self.increment_in_cpu_cycles {
                self.counter = 0;
                if self.tima == 0xFF {
                    self.interrupt = TIMER_INTERRUPT;
                    self.tima = self.tma;
                }
                self.tima += 1;
            }
        }
    }

    pub fn get_timer_control(&self) -> u8 {
        let mut tac = 0b11111000;
        if self.enabled {
            tac |= TIMER_ENABLE;
        }
        tac |= match self.increment_in_cpu_cycles {
            1024 => 0,
            16 => 1,
            64 => 2,
            256 => 3,
            _ => panic!("Invalid Timer clock speed {}", self.increment_in_cpu_cycles)
        };
        tac
    }

    pub fn set_timer_control(&mut self, value: u8) {
        self.enabled = value & TIMER_ENABLE > 0;
        self.increment_in_cpu_cycles = match value & TIMER_CLOCK_SPEED {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => panic!("Invalid Timer clock speed {}", value)
        };
    }
}