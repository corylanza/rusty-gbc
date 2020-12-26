use super::rusty_gbc::{Display, Color, SCREEN_WIDTH, SCREEN_HEIGHT, BYTES_PER_PIXEL};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub struct SdlDisplay<'a> {
    pub canvas: WindowCanvas,
    pub texture: Texture<'a>
}

impl SdlDisplay<'_> {
    pub fn new(canvas: WindowCanvas, tc: &'_ TextureCreator<WindowContext>) -> SdlDisplay<'_> {
        let tex = tc.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::ABGR8888,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        ).unwrap();
        SdlDisplay {
            canvas: canvas,
            texture: tex
        }
    }

    pub fn set_title(&mut self, title: String) {
        let t: &str = &(title.to_owned())[..];
        self.canvas.window_mut().set_title(t).unwrap();
    }
}

impl Display for SdlDisplay<'_> {
    fn render_frame(&mut self) {
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present();
    }

    fn update_line_from_buffer(&mut self, buffer: [Color; SCREEN_WIDTH as usize], pixel_y: u8) {
        self.texture.with_lock(None, |sdl_buffer: &mut [u8], pitch: usize| {
            for pixel_x in 0 .. SCREEN_WIDTH {
                let buf_idx = (pixel_y as usize * pitch) + (pixel_x as usize * BYTES_PER_PIXEL as usize);
                let color = buffer[pixel_x as usize];
                sdl_buffer[buf_idx] = color.r;
                sdl_buffer[buf_idx + 1] = color.g;
                sdl_buffer[buf_idx + 2] = color.b;
                sdl_buffer[buf_idx + 3] = 0xFF;
            }
        }).unwrap();
    }
}