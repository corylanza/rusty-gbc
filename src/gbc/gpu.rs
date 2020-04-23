extern crate sdl2; 

use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const SCREEN_WIDTH: u8 = 160;
const SCREEN_HEIGHT: u8 = 144;

const H_BLANK_MODE: u8 = 0;
const V_BLANK_MODE: u8 = 1;
const OAM_SEARCH_MODE: u8 = 2;
const LCD_TRANSFER_MODE: u8 = 3;
const LCD_STATUS_COINCIDENCE_FLAG: u8 = 4;
const LCD_STATUS_LYC_LY_INTERRUPT_ENABLED: u8 = 64;


const BYTES_PER_PIXEL: u8 = 4; // RGBA8888
const BUFFER_HEIGHT: u16 = 256;
const BUFFER_WIDTH: u16 = 256;
const BUFFER_SIZE: usize =
    (BUFFER_HEIGHT as usize * BUFFER_WIDTH as usize * BYTES_PER_PIXEL as usize);

pub struct Gpu {
    //tileset_canvas: WindowCanvas,
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
    pub wx: u8,
    buffer: Vec<u8>,
    cycle_count: usize,
    pub interrupts: u8
}

type Tile = [[Color; 8]; 8];

fn empty_tile() -> Tile {
    [[Color::RGB(0, 0, 0); 8]; 8]
}

// fn render_tile(canvas: &mut WindowCanvas, tile: Tile, x: usize, y: usize) {
//     for row in 0..8 {
//         for pixel in 0..8 {
//             let real_x = (x as u32 * 8 * SCALE) + (pixel * SCALE);
//             let real_y = (y as u32 * 8 * SCALE) + (row * SCALE);
//             canvas.set_draw_color(tile[row as usize][pixel as usize]);
//             canvas.fill_rect(Rect::new(real_x as i32, real_y as i32, SCALE, SCALE)).unwrap();
//         }
//     }
// }

impl Gpu {
    pub fn new(bg: WindowCanvas) -> Result<Self, String> {
        Ok(Gpu {
            //tileset_canvas: tiles_window.into_canvas().build().unwrap(),
            background_canvas: bg,
            vram: [0; 0x8000],
            tile_set: [empty_tile(); 384],
            lcd_control: 0,
            lcdc_status: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            buffer: vec![0; BUFFER_SIZE],
            cycle_count: 0,
            interrupts: 0
        })
    }

    pub fn gpu_step(&mut self, cycles: u8) {
        self.cycle_count += cycles as usize;

        match self.cycle_count {
            0..=80 => { 
                /* OAM SEARCH */
                self.set_lcdc_mode(OAM_SEARCH_MODE);
            },
            80..=252 => {
                /* SCANLINE*/ 
                if self.lcdc_status & LCD_TRANSFER_MODE != LCD_TRANSFER_MODE {
                    self.set_lcdc_mode(LCD_TRANSFER_MODE);
                    self.draw_scanline();
                }

            },
            252..=456 => {
                /* H-Blank*/ 
                    self.set_lcdc_mode(H_BLANK_MODE);
            },
            _ => {
                self.ly += 1;
                self.cycle_count = 0;

                if self.lcdc_status & LCD_STATUS_LYC_LY_INTERRUPT_ENABLED > 0 && self.lyc == self.ly  {
                    self.lcdc_status |= LCD_STATUS_COINCIDENCE_FLAG;
                    self.interrupts |= 2;
                }

                if self.ly == 154 {
                    self.ly = 0;
                } else if self.ly >= SCREEN_HEIGHT && self.lcdc_status & V_BLANK_MODE != V_BLANK_MODE {
                    self.set_lcdc_mode(V_BLANK_MODE);
                    self.interrupts |= 1;

                    let tc = self.background_canvas.texture_creator();
                    let mut texture = tc.create_texture_streaming(
                        sdl2::pixels::PixelFormatEnum::ABGR8888,
                        256,
                        256,
                    ).unwrap();
    
                    texture.update(
                        None,
                        &*self.buffer,
                        BUFFER_WIDTH as usize * BYTES_PER_PIXEL as usize,
                    ).unwrap();

                    self.background_canvas.copy(&texture, None, None).unwrap();
                    self.background_canvas.set_draw_color(Color::RGB(0, 0, 0));
                    self.background_canvas.draw_rect(Rect::new(self.scx as i32, self.scy as i32, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)).unwrap();
                    self.background_canvas.present();
                }
            }
        }
    }

    fn set_lcdc_mode(&mut self, mode: u8) {
        self.lcdc_status = (self.lcdc_status & 0b11111100) | mode;
    }

    fn draw_scanline(&mut self) {
        for x in 0..32 {
            let tile = self.get_bg_tile_at(x, self.ly / 8);
            for xx in 0..8 {
                let buf_idx = ((self.ly as usize * 256) + ((x as usize * 8) + xx)) * BYTES_PER_PIXEL as usize;
                let color = tile[(self.ly % 8)as usize][xx as usize];
                (*self.buffer)[buf_idx] = color.b;
                (*self.buffer)[buf_idx + 1] = color.g;
                (*self.buffer)[buf_idx + 2] = color.r;
                (*self.buffer)[buf_idx + 3] = color.a;
            }
        }
    }

    // pub fn render_background(&mut self) {
    //     self.background_canvas.set_draw_color(Color::RGB(0, 0xFF, 0xFF));
    //     self.background_canvas.clear();
    //     for tile_y in 0..32 {
    //         for tile_x in 0..32 {
    //             let tile = self.get_bg_tile_at(tile_y, tile_x);
    //             render_tile(&mut self.background_canvas, tile, tile_x as usize, tile_y as usize);
    //         }
    //     }
    //     self.background_canvas.set_draw_color(Color::RGB(0, 0, 0));
    //     self.background_canvas.draw_rect(Rect::new(self.scx as i32 * SCALE as i32, self.scy as i32 * SCALE as i32, SCREEN_WIDTH as u32 * SCALE as u32, SCREEN_HEIGHT as u32 * SCALE as u32)).unwrap();
    //     self.background_canvas.present();
    // }


    // fn render_tileset(&mut self) {
    //     self.tileset_canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
    //     self.tileset_canvas.clear();
    //     for tile in 0..384 {
    //         render_tile(&mut self.tileset_canvas, self.tile_set[tile], tile % 16, tile / 16);
    //     }
    //     self.tileset_canvas.present();
    // }

    fn get_bg_tile_at(&self, x: u8, y: u8) -> Tile {
        let address = y as u16 * 32 + x as u16;
        let tile_id = if self.lcd_control & 0b00001000 > 0 {
            self.vram[(address + 0x1C00) as usize]
        } else {
            self.vram[(address + 0x1800) as usize]
        };

        if self.lcd_control & 0b00010000 > 0 {
            self.tile_set[tile_id as usize]
        } else {
            let address = (0x100 as i16) + i8::from_le_bytes([tile_id]) as i16;
            self.tile_set[address as usize]
        }
    }

    pub fn write_to_vram(&mut self, address: u16, value: u8) {
        if self.lcd_control & LCD_TRANSFER_MODE == LCD_TRANSFER_MODE {
            // cannot access VRAM during LCD Transfer
            return;
        }
        self.vram[address as usize] = value;
        if address >= 0x1800 {
            return; 
        }

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
        if self.lcd_control & LCD_TRANSFER_MODE == LCD_TRANSFER_MODE {
            // cannot access VRAM during LCD Transfer
            return 0xFF;
        }
        self.vram[address as usize]
    }

    pub fn write_to_oam(&mut self, value: u8) {
        if self.lcd_control & OAM_SEARCH_MODE == LCD_TRANSFER_MODE
            ||  self.lcd_control & LCD_TRANSFER_MODE == LCD_TRANSFER_MODE {
            // cannot access OAM during OAM search or LCD Transfer
            return;
        }
    }

    pub fn read_from_oam(&mut self) -> u8 {
        if self.lcd_control & OAM_SEARCH_MODE == LCD_TRANSFER_MODE
            ||  self.lcd_control & LCD_TRANSFER_MODE == LCD_TRANSFER_MODE {
            // cannot access OAM during OAM search or LCD Transfer
            return 0xFF;
        }
        0xff
    }
}