use sdl2::pixels::Color;
use std::time::Instant;

use crate::Display;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use super::{V_BLANK_INTERRUPT, STAT_INTERRUPT};

const SPRITE_X_LIM: u8 = SCREEN_WIDTH + 8;
const SPRITE_Y_LIM: u8 = SCREEN_HEIGHT + 16;

//const SPRITE_OBJ_TO_BG_PRIORITY: u8 = 0b01000000; // (0=OBJ Above BG, 1=OBJ Behind BG color 1-3) //(Used for both BG and Window. BG color 0 is always behind OBJ)
const SPRITE_Y_FLIP: u8 = 0b01000000; // (0=Normal, 1=Vertically mirrored)
const SPRITE_X_FLIP: u8 = 0b00100000; //(0=Normal, 1=Horizontally mirrored)
const SPRITE_PALETTE_NUM: u8 = 0b00010000; // **Non CGB Mode Only** (0=OBP0, 1=OBP1)

const H_BLANK_MODE: u8 = 0;
const V_BLANK_MODE: u8 = 1;
const OAM_SEARCH_MODE: u8 = 2;
const LCD_TRANSFER_MODE: u8 = 3;

const BYTES_PER_PIXEL: u8 = 4; // RGBA8888

const WHITE: Color = Color::RGB(0xE6, 0xFF, 0xE6);
const LIGHT_GRAY: Color = Color::RGB(0x00, 0x80, 0x40);
const DARK_GRAY: Color = Color::RGB(0x70, 0xDB, 0x70);
const BLACK: Color = Color::RGB(0x00, 0x00, 0x00);

pub struct Gpu {
    vram: [u8; 0x8000],
    oam: [u8; 0xA0],
    tile_set: [Tile; 384],
    sprites: [Option<Sprite>; 10],
    // LCD Control
    lcd_enable: bool,
    window_tile_map: bool,
    window_enable: bool,
    /// BG & Window Tile Data Select (false=8800-97FF, true=8000-8FFF)
    bg_window_tile_data: bool,
    /// BG Tile Map Display Select (false=9800-9BFF, true=9C00-9FFF)
    bg_tile_map_select: bool,
    double_sprite_size: bool,
    sprite_enable: bool,
    bg_window_priority: bool,
    // LCDC Status
    coincidence_interrupt_enabled: bool,
    oam_interrupt_enabled: bool,
    v_blank_interrupt_enabled: bool,
    h_blank_interrupt_enabled: bool,
    coincidence_flag: bool,
    lcd_mode: u8,
    // Misc
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    wy: u8,
    wx: u8,            
    bgp: u8,
    obp0: u8,
    obp1: u8,
    cycle_count: usize,
    pub interrupts: u8,
    updated: bool,
    framecount: u32,
    timer: Instant
}

type Tile = [[u8; 8]; 8];

struct Sprite {
    y: u8,
    x: u8,
    tile_number: u8,
    flags: u8
}

fn empty_tile() -> Tile {
    [[0; 8]; 8]
}

impl Gpu {
    pub fn new() -> Result<Self, String> {
        Ok(Gpu {
            oam: [0; 0xA0],
            vram: [0; 0x8000],
            tile_set: [empty_tile(); 384],
            sprites: Default::default(),
            // LCD Control
            lcd_enable: false,
            window_tile_map: false,
            window_enable: false,
            bg_window_tile_data: false,
            bg_tile_map_select: false,
            double_sprite_size: false,
            sprite_enable: false,
            bg_window_priority: false,
            // LCDC Status
            coincidence_interrupt_enabled: false,
            oam_interrupt_enabled: false,
            v_blank_interrupt_enabled: false,
            h_blank_interrupt_enabled: false,
            coincidence_flag: false,
            lcd_mode: 0,
            // Misc
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            cycle_count: 0,
            interrupts: 0,
            updated: true,
            framecount: 0,
            timer: Instant::now()
        })
    }

    pub fn gpu_step(&mut self, display: &mut Display, cycles: u8) {
        if !self.lcd_enable {
            return;
        }
        self.cycle_count += cycles as usize;
        let elapsed = self.timer.elapsed().as_millis();
        if elapsed > 1000 {
            self.timer = Instant::now();
            println!("fps {}", self.framecount as f32 / (elapsed as f32 / 1000.0));
            self.framecount = 0;
        }

        if self.coincidence_interrupt_enabled && self.lyc == self.ly  {
            self.coincidence_flag = true;
            self.interrupts |= STAT_INTERRUPT;
            // TODO this likely shouldn't request an interrupt at every step
        }

        if self.ly < SCREEN_HEIGHT {
            match self.cycle_count {
                0..=80 => {
                    /* OAM SEARCH */
                    if self.get_lcdc_mode() != OAM_SEARCH_MODE {
                        //self.sprites = self.get_sprite(n: u8)
                        self.set_lcdc_mode(OAM_SEARCH_MODE);
                        if self.oam_interrupt_enabled {
                            self.interrupts |= STAT_INTERRUPT;
                        }
                    }
                },
                80..=252 => {
                    /* SCANLINE*/ 
                    if self.get_lcdc_mode() != LCD_TRANSFER_MODE {
                        self.set_lcdc_mode(LCD_TRANSFER_MODE);
                        if self.updated {
                            self.draw_scanline(display);
                        }
                    }
                },
                252..=456 => {
                    /* H-Blank*/ 
                    if self.get_lcdc_mode() != H_BLANK_MODE {
                        self.set_lcdc_mode(H_BLANK_MODE);
                        if self.h_blank_interrupt_enabled {
                            self.interrupts |= STAT_INTERRUPT;
                        }
                    }

                },
                _ => {
                    self.ly += 1;
                    self.cycle_count = 0;
                }
            };
        } else {
            if self.get_lcdc_mode() != V_BLANK_MODE {
                self.set_lcdc_mode(V_BLANK_MODE);
                self.interrupts |= V_BLANK_INTERRUPT;
                
                self.framecount += 1;
                
                if self.updated {
                    self.display_sprites(display);
                    display.render_frame();
                }
                
                self.updated = false;
            }
            if self.cycle_count > 456 {
                self.ly += 1;
                self.cycle_count = 0;
            }
            if self.ly > SCREEN_HEIGHT + 10 {
                self.ly = 0;
            }
        }
    }

    fn updated(&mut self) {
        //println!("updated");
        self.updated = true;
    }

    pub fn set_lcdc_control(&mut self, value: u8) {
        let bit = |flag: u8| value & 1 << flag > 0;
        self.lcd_enable = bit(7);
        self.window_tile_map = bit(6);
        self.window_enable = bit(5);
        self.bg_window_tile_data = bit(4);
        self.bg_tile_map_select = bit(3);
        self.double_sprite_size = bit(2);
        self.sprite_enable = bit(1);
        self.bg_window_priority = bit(0);
        self.updated() 
    }
    pub fn get_lcdc_control(&self) -> u8 {
        let mut lcdc = 0;
        let mut bit = |flag: u8, cond: bool| if cond { lcdc |= 1 << flag };
        bit(7, self.lcd_enable);
        bit(6, self.window_tile_map);
        bit(5, self.window_enable);
        bit(4, self.bg_window_tile_data);
        bit(3, self.bg_tile_map_select);
        bit(2, self.double_sprite_size);
        bit(1, self.sprite_enable);
        bit(0, self.bg_window_priority);
        lcdc
    }

    pub fn set_lcdc_status(&mut self, value: u8) {
        let bit = |flag: u8| value & 1 << flag > 0;
        self.coincidence_interrupt_enabled = bit(7);
        self.oam_interrupt_enabled = bit(6);
        self.v_blank_interrupt_enabled = bit(5);
        self.h_blank_interrupt_enabled = bit(4);
        self.updated() 
    }

    pub fn get_lcdc_status(&self) -> u8 {
        let mut stat = 0b10000000; // Unused bit 7 always 1
        let mut bit = |flag: u8, cond: bool, | if cond { stat |= 1 << flag };
        bit(6, self.coincidence_interrupt_enabled);
        bit(5, self.oam_interrupt_enabled);
        bit(4, self.v_blank_interrupt_enabled);
        bit(3, self.h_blank_interrupt_enabled);
        bit(2, self.coincidence_flag);
        stat |= self.get_lcdc_mode();
        stat
    }

    fn set_lcdc_mode(&mut self, mode: u8) {
        self.lcd_mode = mode & 0b11;
    }

    fn get_lcdc_mode(&self) -> u8 {
        self.lcd_mode & 0b11
    }

    pub fn set_scy(&mut self, value: u8) { self.scy = value; self.updated() }
    pub fn get_scy(&self) -> u8 { self.scy }

    pub fn set_scx(&mut self, value: u8) { self.scx = value;  self.updated() }
    pub fn get_scx(&self) -> u8 { self.scx }

    pub fn get_ly(&self) -> u8 { self.ly }

    pub fn set_lyc(&mut self, value: u8) { self.lyc = value; self.updated() }
    pub fn get_lyc(&self) -> u8 { self.lyc }

    pub fn set_wy(&mut self, value: u8) { self.wy = value; self.updated() }
    pub fn get_wy(&self) -> u8 { self.wy }

    pub fn set_wx(&mut self, value: u8) { self.wx = value; self.updated() }
    pub fn get_wx(&self) -> u8 { self.wx }

    pub fn set_bgp(&mut self, value: u8) { self.bgp = value; self.updated() }
    pub fn get_bgp(&self) -> u8 { self.bgp }

    pub fn set_obp0(&mut self, value: u8) { self.obp0 = value; self.updated() }
    pub fn get_obp0(&self) -> u8 { self.obp0 }

    pub fn set_obp1(&mut self, value: u8) { self.obp1 = value; self.updated() }
    pub fn get_obp1(&self) -> u8 { self.obp1 }




    fn draw_scanline(&mut self, display: &mut Display) {
        display.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let y = self.ly.wrapping_add(self.scy);
            for x in 0u8..32 {
                let x = x * 8;
                let tile = self.get_bg_tile_at(x / 8, y / 8);
                for xx in 0..8 {
                    let x_pix = x.wrapping_add(xx).wrapping_sub(self.scx);
                    if x_pix < SCREEN_WIDTH && self.ly < SCREEN_HEIGHT {
                        let buf_idx = (self.ly as usize * pitch) + (x_pix as usize * BYTES_PER_PIXEL as usize);
                        let color = self.get_bg_color(tile[(y % 8)as usize][xx as usize]);
                        
                        buffer[buf_idx] = color.b;
                        buffer[buf_idx + 1] = color.g;
                        buffer[buf_idx + 2] = color.r;
                        buffer[buf_idx + 3] = color.a;
                    }
                }
            }
        }).unwrap();
        
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

    // pub fn render_tileset(&mut self, tileset_canvas: &mut WindowCanvas) {
    //     tileset_canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
    //     tileset_canvas.clear();
    //     for tile in 0..384 {
    //         render_tile(tileset_canvas, self.tile_set[tile], tile % 16, tile / 16, 2);
    //     }
    //     tileset_canvas.present();
    // }

    fn get_bg_tile_at(&self, x: u8, y: u8) -> Tile {
        let address = y as u16 * 32 + x as u16;
        let tile_id = if self.bg_tile_map_select {
            self.vram[(address + 0x1C00) as usize]
        } else {
            self.vram[(address + 0x1800) as usize]
        };

        if self.bg_window_tile_data {
            self.tile_set[tile_id as usize]
        } else {
            let address = (0x100 as i16) + i8::from_le_bytes([tile_id]) as i16;
            self.tile_set[address as usize]
        }
    }

    fn display_sprites(&mut self, display: &mut Display) {
        for i in 0 .. 40 {
            let sprite = self.get_sprite(i);
            match (sprite.x, sprite.y) {
                (1 ..= SPRITE_X_LIM, 1 ..= SPRITE_Y_LIM) => {
                    let tile = self.get_sprite_tile(sprite.tile_number);
                    display.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                        let flip = sprite.flags & SPRITE_X_FLIP == SPRITE_X_FLIP;
                        for x in 0..8 {
                            let sprite_x = if flip { 
                                sprite.x.wrapping_sub(x +1) as usize
                            } else {
                                sprite.x.wrapping_sub(8).wrapping_add(x) as usize
                            };
                            
                            for y in 0..8 {
                                let sprite_y = if sprite.flags & SPRITE_Y_FLIP == SPRITE_Y_FLIP {
                                    sprite.y.wrapping_sub(8).wrapping_sub(y) as usize
                                } else {
                                    sprite.y.wrapping_sub(16).wrapping_add(y) as usize
                                };
                                if  sprite_x < SCREEN_WIDTH as usize && sprite_y < SCREEN_HEIGHT as usize {
                                    let buf_idx = ((sprite_y) * pitch) + ((sprite_x)* BYTES_PER_PIXEL as usize);
                                    let color = self.get_sprite_color(sprite.flags & SPRITE_PALETTE_NUM > 0,tile[y as usize][x as usize]);
                                    match color {
                                        None => {},
                                        Some(color) => {
                                            buffer[buf_idx] = color.b;
                                            buffer[buf_idx + 1] = color.g;
                                            buffer[buf_idx + 2] = color.r;
                                            buffer[buf_idx + 3] = color.a;
                                        }
                                    }
                                }
                            }
                        }
                    }).unwrap();
                },
                _ => {}
            }
        }
    }

    fn get_sprite_tile(&self, tile_number: u8) -> Tile {
        self.tile_set[tile_number as usize]
    }

    fn get_sprite(&self, n: u8) -> Sprite {
        let idx = n * 4;
        let sprite = &self.oam[idx as usize .. (idx + 4) as usize];
        Sprite {
            y: sprite[0],
            x: sprite[1],
            tile_number: sprite[2],
            flags: sprite[3]
        }

    }

    pub fn write_to_vram(&mut self, address: u16, value: u8) {
        self.updated(); 
        // if self.lcdc_status & 0b00000011 == LCD_TRANSFER_MODE {
        //     // cannot access VRAM during LCD Transfer
        //     return;
        // }
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
            let color = ((byte1 & mask) >> (7 - pixel)) << 1 | ((byte2 & mask) >> (7 - pixel));
            self.tile_set[tile][row][pixel] = color;
        }
    }

    fn get_sprite_color(&self, useobp1: bool, value: u8) -> Option<Color> {
        let palette = if useobp1 { self.obp1 } else { self.obp0 };
        if value == 0 {
            return None;
        }
        match (palette & (0b11 << (2 * value))) >> (2 * value) {
            0 => Some(WHITE),
            1 => Some(LIGHT_GRAY),
            2 => Some(DARK_GRAY),
            3 => Some(BLACK),
            _ => None
        }
    }

    fn get_bg_color(&self, value: u8) -> Color {
        match (self.bgp & (0b11 << (2 * value))) >> (2 * value) {
            0 => WHITE,
            1 => LIGHT_GRAY,
            2 => DARK_GRAY,
            3 => BLACK,
            _ => Color::RGB(0, 0, 0xff)
        }
    }

    pub fn read_from_vram(&self, address: u16) -> u8 {
        if self.get_lcdc_mode() == LCD_TRANSFER_MODE {
            // cannot access VRAM during LCD Transfer
            return 0xFF;
        }
        self.vram[address as usize]
    }

    pub fn write_to_oam(&mut self, address: u16, value: u8) {
        self.updated();
        if false /*self.lcdc_status & 0b00000011 == OAM_SEARCH_MODE
            ||  self.lcdc_status & 0b00000011 == LCD_TRANSFER_MODE */ {
            // cannot access OAM during OAM search or LCD Transfer
            return;
        } else {
            self.oam[address as usize] = value;
        }
    }

    pub fn read_from_oam(&self, address: u16) -> u8 {
        if false /*self.lcdc_status & 0b00000011 == OAM_SEARCH_MODE
            ||  self.lcdc_status & 0b00000011 == LCD_TRANSFER_MODE*/ {
            // cannot access OAM during OAM search or LCD Transfer
            0xFF
        } else {
            self.oam[address as usize]
        }
    }
}