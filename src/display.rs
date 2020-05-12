use sdl2::render::Texture;

pub struct Display<'a> {
    pub texture: Texture<'a>,
    width: u16,
    height: u16
}

impl Display<'_> {
    pub fn new(texture: Texture<'_>, width: u16, height: u16) -> Display<'_> {
        Display {
            texture: texture,
            width: width,
            height: height
        }
    }
}