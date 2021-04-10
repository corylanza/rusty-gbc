pub mod gbc;
pub mod debugger;

pub const SCREEN_WIDTH: u8 = 160;
pub const SCREEN_HEIGHT: u8 = 144;
pub const BYTES_PER_PIXEL: u8 = 4; // RGBA8888
pub const PIXEL_BUFFER_SIZE: usize =  SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize * BYTES_PER_PIXEL as usize;

#[derive(Default, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: r,
            g: g,
            b: b
        }
    }
}

pub trait Display {
    fn render_frame(&mut self, buffer: &mut [u8; PIXEL_BUFFER_SIZE as usize]);
}