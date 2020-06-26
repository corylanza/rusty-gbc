extern crate rusty_gbc;

use rusty_gbc::gbc::Cpu;
use rusty_gbc::debugger::Debugger;
use rusty_gbc::{SCREEN_HEIGHT, SCREEN_WIDTH};
use rusty_gbc::gbc::gpu::Gpu;
use std::env;

extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use std::fs::File;
use std::io::prelude::*;
//use std::time::Instant;

mod display;
use display::SdlDisplay;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Rusty GBC", SCREEN_WIDTH as u32 * 3, SCREEN_HEIGHT as u32 * 3)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let canvas = window.into_canvas()
            .target_texture()
            .present_vsync()
            .accelerated()
            .build()
            .unwrap();

        let tc = canvas.texture_creator();
        let mut display = SdlDisplay::new(canvas, &tc);

        // let tiles_window = video_subsystem.window("Tileset", 16 * 8 * 2, (384 / 16) * 8 * 2)
        //     .position_centered()
        //     .opengl()
        //     .resizable()
        //     .build()
        //     .unwrap();
        // let mut tile_canvas = tiles_window.into_canvas()
        //     .target_texture()
        //     .present_vsync()
        //     .accelerated()
        //     .build()
        //     .unwrap(); 

        let mut file = File::open(&args[1]).unwrap();
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).unwrap();
        
        let color_mode = buffer[0x143] & 0x80 == 0x80 || buffer[0x143] & 0xC0 == 0xC0;
        let gpu = Gpu::new(color_mode).unwrap();
        let mut gbc = Cpu::new(buffer, gpu);

        if args.len() > 2 {
            let debugger = Debugger::new(&args[2]);
            gbc.attatch_debugger(debugger);
        }
        
        let mut event_pump = sdl_context.event_pump()?;

        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'main
                    },
                    Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                        gbc.log = !gbc.log;
                    },
                    Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                        gbc.mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::A);
                    },
                    Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                        gbc.mem.input.key_released(rusty_gbc::gbc::input::Keycode::A);
                    },
                    Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                        gbc.mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::B);
                    },
                    Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                        gbc.mem.input.key_released(rusty_gbc::gbc::input::Keycode::B);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                        gbc.mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::Start);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Return), .. } => {
                        gbc.mem.input.key_released(rusty_gbc::gbc::input::Keycode::Start);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                        gbc.mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::Select);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                        gbc.mem.input.key_released(rusty_gbc::gbc::input::Keycode::Select);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                        gbc.mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::Left);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                        gbc.mem.input.key_released(rusty_gbc::gbc::input::Keycode::Left);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        gbc.mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::Right);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                        gbc.mem.input.key_released(rusty_gbc::gbc::input::Keycode::Right);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                        gbc.mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::Down);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                        gbc.mem.input.key_released(rusty_gbc::gbc::input::Keycode::Down);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                        gbc.mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::Up);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                        gbc.mem.input.key_released(rusty_gbc::gbc::input::Keycode::Up);
                    },
                    Event::MouseButtonDown { .. } => {
                        println!("PC: {:04X} op {:02X}", gbc.regs.pc, gbc.mem.read(gbc.regs.pc));
                    }
                    _ => {}
                }
            }
            
            gbc.run_one_frame(&mut display);
            // To reintroduce fps count use below
            // let elapsed = self.timer.elapsed().as_millis();
            // if elapsed > 1000 {
            //     self.timer = Instant::now();
            //     println!("fps {}", self.framecount as f32 / (elapsed as f32 / 1000.0));
            //     self.framecount = 0;
            // }
        }
        Ok(())
    } else {
        panic!("No cartridge found");
    }
}
