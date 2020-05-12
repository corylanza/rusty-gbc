use sdl2::render::WindowCanvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct Display<'a> {
    pub texture: Texture<'a>,
    width: u32,
    height: u32
}

impl Display<'_> {
    pub fn new(tc: &'_ TextureCreator<WindowContext>, width: u32, height: u32) -> Display<'_> {
        let tex = tc.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::ABGR8888,
            width,
            height,
        ).unwrap();
        Display {
            texture: tex,
            width: width,
            height: height
        }
    }

    pub fn update_buffer(&mut self) {
        self.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {

        }).unwrap();
    }
}