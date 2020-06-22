use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d, ImageData};
use rusty_gbc::{Display, Color, SCREEN_WIDTH, SCREEN_HEIGHT};
use stdweb::js;

#[derive(Clone)]
pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    image_data: ImageData,
    scaled_width: u32,
    scaled_height: u32
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
        let image_data = ctx.create_image_data(width as f64, height as f64).unwrap();

        let scaled_width = width / SCREEN_WIDTH as u32;
        let scaled_height = height / SCREEN_HEIGHT as u32;

        Canvas {
            canvas,
            ctx,
            image_data,
            scaled_width,
            scaled_height
        }
    }

    fn draw(&self, x: u32, y: u32, color: String) {
        assert!(x < SCREEN_WIDTH as u32);
        assert!(y < SCREEN_HEIGHT as u32);

        js!({
            @{&self.ctx}.fillStyle = @{color};
        });

        let x = x * self.scaled_width;
        let y = y * self.scaled_height;

        self.ctx.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(self.scaled_width),
            f64::from(self.scaled_height),
        );
    }
}

impl Display for Canvas {
    fn render_frame(&mut self) {

    }

    fn update_line_from_buffer(&mut self, buffer: [Color; SCREEN_WIDTH as usize], line: u8) {
        for pixel_x in 0..SCREEN_WIDTH {
            let color = buffer[pixel_x as usize];
            self.draw(pixel_x as u32, line as u32, format!("rgb({},{},{})", color.r, color.g, color.b));
        }
    }
}