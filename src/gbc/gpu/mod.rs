extern crate sdl2; 

use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Gpu {
    sdl_context: Sdl,
    canvas: WindowCanvas,
    vram: [u8; 0x8000],
    tile_set: [Tile; 384],
}

type Tile = [[Color; 8]; 8];
const SCALE: u32 = 4;

fn empty_tile() -> Tile {
    [[Color::RGB(0, 0, 0); 8]; 8]
}

impl Gpu {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
     
        let window = video_subsystem.window("Rusty GBC", 16 * 8 * SCALE, (384 / 16) * 8 * SCALE)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        Gpu {
            sdl_context: sdl,
            canvas: window.into_canvas().present_vsync().build().unwrap(),
            vram: [0; 0x8000],
            tile_set: [empty_tile(); 384]
        }
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        self.canvas.clear();
        for tile in 0..384 {
            for row in 0..8 {
                for pixel in 0..8 {
                    let x: usize = ((tile % 16) * (8 * SCALE as usize)) + (pixel * SCALE as usize);
                    let y: usize = ((tile / 16) * (8 * SCALE as usize)) + (row * SCALE as usize);
                    self.canvas.set_draw_color(self.tile_set[tile][row][pixel]);//Color::RGB(((tile * 16) % 256) as u8, 0x00, 0x00));
                    self.canvas.fill_rect(Rect::new(x as i32, y as i32, SCALE, SCALE)).unwrap();
                }
            }
        }
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    panic!("closed");
                },
                _ => {}
            }
        }
        self.canvas.present();
    }

    pub fn write_to_vram(&mut self, address: u16, value: u8) {
        self.vram[address as usize] = value;
        if address >= 0x1800 { return; }
        
        let index: usize = (address & 0xFFFE) as usize;
        let byte1 = self.vram[index];
        let byte2 = self.vram[index + 1];
        let tile: usize = (index / 16) as usize;
        let row: usize = ((index % 16) / 2) as usize;

        for pixel in 0..8 {
            let mask = 1 << (7 - pixel);
            self.tile_set[tile][row][pixel] = match (byte1 & mask != 0, byte2 & mask != 0) {
                (true, true) => Color::RGB(0x00, 0x00, 0x00),
                (false, true) => Color::RGB(0x00, 0x80, 0x40),
                (true, false) => Color::RGB(0x70, 0xDB, 0x70),
                (false, false) => Color::RGB(0xE6, 0xFF, 0xE6),
            };
        }
    }

    pub fn read_from_vram(&self, address: u16) -> u8 {
        self.vram[address as usize]
    }
}