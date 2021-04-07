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
    //pixel_buffer: [u8; PIXEL_BUFFER_SIZE],
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
            //pixel_buffer: [0; PIXEL_BUFFER_SIZE],
            scaled_width,
            scaled_height,
            frame_count: 0,
        }
    }
}

impl Display for Canvas {
    fn render_frame(&mut self, buffer: &mut [u8; PIXEL_BUFFER_SIZE]) {
        //self.frame_count = if self.frame_count > 6 { 0 } else { self.frame_count + 1 };
        self.ctx.put_image_data(self.image_data(buffer), 0.0, 0.0).unwrap();
    }

    fn update_line_from_buffer(&mut self, buffer: [Color; SCREEN_WIDTH as usize], pixel_y: u8) {
        if self.frame_count == 0 {
            // for pixel_x in 0..SCREEN_WIDTH {
            //     let color = buffer[pixel_x as usize];
            //     let buf_idx = ((pixel_y as usize * SCREEN_WIDTH as usize) + (pixel_x as usize)) * BYTES_PER_PIXEL as usize;
            //     self.pixel_buffer[buf_idx] = color.r;
            //     self.pixel_buffer[buf_idx + 1] = color.g;
            //     self.pixel_buffer[buf_idx + 2] = color.b;
            //     self.pixel_buffer[buf_idx + 3] = 0xFF;
            // }
        }
    }
}

impl Canvas {
    fn image_data(&self, pixels: &[u8; PIXEL_BUFFER_SIZE]) -> ImageData {
        let array: Array = Array::from(pixels.to_vec());
        js!({
            return create_frame_image_data(@{array});
        }).try_into().unwrap()
    }
}