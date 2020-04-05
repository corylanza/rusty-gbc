extern crate sdl2; 

use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// const SCREEN_WIDTH: u8 = 160;
// const SCREEN_HEIGHT: u8 = 144;

const H_BLANK_MODE: u8 = 0;
const V_BLANK_MODE: u8 = 1;
// const OAM_SEARCH_MODE: u8 = 2;
// const LCD_TRANSFER_MODE: u8 = 3;

pub struct Gpu {
    sdl_context: Sdl,
    tileset_canvas: WindowCanvas,
    background_canvas: WindowCanvas,
    vram: [u8; 0x8000],
    tile_set: [Tile; 384],
    pub lcd_control: u8,
    pub lcdc_status: u8,
    pub scy: u8,
    pub scx: u8,
    pub ly: u8,
    pub lyc: u8,
    pub wy: u8,
    pub wx: u8
}

type Tile = [[Color; 8]; 8];
const SCALE: u32 = 2;

fn empty_tile() -> Tile {
    [[Color::RGB(0, 0, 0); 8]; 8]
}

fn render_tile(canvas: &mut WindowCanvas, tile: Tile, x: usize, y: usize) {
    for row in 0..8 {
        for pixel in 0..8 {
            let real_x = (x as u32 * 8 * SCALE) + (pixel * SCALE);
            let real_y = (y as u32 * 8 * SCALE) + (row * SCALE);
            canvas.set_draw_color(tile[row as usize][pixel as usize]);
            canvas.fill_rect(Rect::new(real_x as i32, real_y as i32, SCALE, SCALE)).unwrap();
        }
    }
}

impl Gpu {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
     
        let tiles_window = video_subsystem.window("Tileset", 16 * 8 * SCALE, (384 / 16) * 8 * SCALE)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let bg_window = video_subsystem.window("Background", 32 * 8 * SCALE, 32 * 8 * SCALE)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        Gpu {
            sdl_context: sdl,
            tileset_canvas: tiles_window.into_canvas().present_vsync().build().unwrap(),
            background_canvas: bg_window.into_canvas().present_vsync().build().unwrap(),
            vram: [0; 0x8000],
            tile_set: [empty_tile(); 384],
            lcd_control: 0,
            lcdc_status: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0
        }
    }

    fn set_lcdc_mode(&mut self, mode: u8) {
        self.lcdc_status = self.lcdc_status & 0b11111100 | mode;
    }

    pub fn render_scanline(&mut self) -> u64 {
        self.ly = (self.ly + 1) % 154;
        match self.ly {
            0..=143 => {
                // OAM search 20 * cycles
                // Pixel Transfer 43 * 4 cycles
                // H - blank 51 * 4 cycles
                self.set_lcdc_mode(H_BLANK_MODE);
                51 * 4
            },
            144..=153 => {
                
                if self.ly == 144 {
                    self.render();
                }
                self.set_lcdc_mode(V_BLANK_MODE);
                114 * 4
            },
            _ => {
                0
            }
        }
    }

    // pub fn draw_scanline(&mut self, scanline: u8) {
    //     for x in 0..SCREEN_WIDTH {
    //         self.get_tile_at(scanline / 8 * 32 + )
    //     }
    // }

    pub fn render(&mut self) {
        self.render_tileset();
        self.render_background();
    }

    fn render_background(&mut self) {
        self.background_canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        self.background_canvas.clear();
        for tile_y in 0..32 {
            for tile_x in 0..32 {
                let tile = self.get_tile_at((tile_y * 32) + tile_x);
                render_tile(&mut self.background_canvas, tile, tile_x as usize, tile_y as usize);
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
        self.background_canvas.present();
    }


    fn render_tileset(&mut self) {
        self.tileset_canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        self.tileset_canvas.clear();
        for tile in 0..384 {
            render_tile(&mut self.tileset_canvas, self.tile_set[tile], tile % 16, tile / 16);
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
        self.tileset_canvas.present();
    }

    fn get_tile_at(&self, address: u16) -> Tile {
        let tile_id = self.vram[(address + 0x1800) as usize];
        if self.lcd_control & 0b00010000 > 0 {
            self.tile_set[tile_id as usize]
        } else {
            let address = (0x100 as i16) + i8::from_le_bytes([tile_id]) as i16;
            self.tile_set[address as usize]
        }
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