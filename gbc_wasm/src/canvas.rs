use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d, ImageData};
use rusty_gbc::{Display, Color, SCREEN_WIDTH, SCREEN_HEIGHT, BYTES_PER_PIXEL};
use stdweb::{js, Array};

const PIXEL_BUFFER_SIZE: usize =  SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize * BYTES_PER_PIXEL as usize;

#[derive(Clone)]
pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    frame_count: u8,
}

impl Canvas {
    pub fn new(attr_id: &str) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        
        let width = canvas.width();
        let height = canvas.height();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let scaled_width = width / SCREEN_WIDTH as u32;
        let scaled_height = height / SCREEN_HEIGHT as u32;

        Canvas {
            canvas,
            ctx,
            scaled_width,
            scaled_height,
            frame_count: 0,
        }
    }
}

impl Display for Canvas {
    fn render_frame(&mut self, buffer: &mut [u8; PIXEL_BUFFER_SIZE]) {
        //self.ctx.put_image_data(self.image_data(buffer), 0.0, 0.0).unwrap();
        self.put_image_data(buffer);
    }
}

impl Canvas {
    fn image_data(&self, pixels: &[u8; PIXEL_BUFFER_SIZE]) -> ImageData {
        let array: Array = Array::from(pixels.to_vec());
        js!({
            return create_frame_image_data(@{array});
        }).try_into().unwrap()
    }

    fn put_image_data(&self, pixels: &[u8; PIXEL_BUFFER_SIZE]) {
        let array: Array = Array::from(pixels.to_vec());
        js!({
            update_frame_image_data(@{array});
        });
    }
}