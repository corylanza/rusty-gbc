pub mod gbc;
pub mod debugger;

pub const SCREEN_WIDTH: u8 = 160;
pub const SCREEN_HEIGHT: u8 = 144;

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
    fn render_frame(&mut self);
    fn update_line_from_buffer(&mut self, buffer: [Color; SCREEN_WIDTH as usize], line: u8);
}