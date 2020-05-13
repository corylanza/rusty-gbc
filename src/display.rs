use sdl2::render::WindowCanvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub const SCREEN_WIDTH: u8 = 160;
pub const SCREEN_HEIGHT: u8 = 144;

pub struct Display<'a> {
    pub canvas: WindowCanvas,
    pub texture: Texture<'a>,
    //width: u32,
    //height: u32
}

impl Display<'_> {
    pub fn new(canvas: WindowCanvas, tc: &'_ TextureCreator<WindowContext>, width: u32, height: u32) -> Display<'_> {
        let tex = tc.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::ABGR8888,
            width,
            height,
        ).unwrap();
        Display {
            canvas: canvas,
            texture: tex,
            //width: width,
            //height: height
        }
    }

    pub fn render_frame(&mut self) {
        self.canvas.copy(&self.texture, None, None).unwrap();
        //self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        //self.canvas.draw_rect(Rect::new(scx as i32, scy as i32, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)).unwrap();
        self.canvas.present();
    }
}