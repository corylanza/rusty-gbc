use std::env;
mod gbc;
mod debugger;
mod display;

use gbc::Cpu;
use debugger::Debugger;
use display::{Display, SCREEN_HEIGHT, SCREEN_WIDTH};
use gbc::gpu::Gpu;

extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;
use sdl2::event::Event;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let bg_window = video_subsystem.window("Background", 256, 256)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let canvas = bg_window.into_canvas()
            .target_texture()
            .present_vsync()
            .accelerated()
            .build()
            .unwrap();

        let tc = canvas.texture_creator();
        let mut display = Display::new(canvas, &tc, 256, 256);

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

        let gpu = Gpu::new().unwrap();
        let mut gbc = Cpu::new(&args[1], gpu);

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
                    // Event::KeyDown { keycode: Some(Keycode::T), .. } => {
                    //     gbc.mem.gpu.render_tileset(&mut tile_canvas);
                    // },
                    Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                        gbc.mem.input.key_pressed(gbc::input::Keycode::A);
                    },
                    Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                        gbc.mem.input.key_released(gbc::input::Keycode::A);
                    },
                    Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                        gbc.mem.input.key_pressed(gbc::input::Keycode::B);
                    },
                    Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                        gbc.mem.input.key_released(gbc::input::Keycode::B);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                        gbc.mem.input.key_pressed(gbc::input::Keycode::Start);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Return), .. } => {
                        gbc.mem.input.key_released(gbc::input::Keycode::Start);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                        gbc.mem.input.key_pressed(gbc::input::Keycode::Select);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                        gbc.mem.input.key_released(gbc::input::Keycode::Select);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                        gbc.mem.input.key_pressed(gbc::input::Keycode::Left);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                        gbc.mem.input.key_released(gbc::input::Keycode::Left);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        gbc.mem.input.key_pressed(gbc::input::Keycode::Right);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                        gbc.mem.input.key_released(gbc::input::Keycode::Right);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                        gbc.mem.input.key_pressed(gbc::input::Keycode::Down);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                        gbc.mem.input.key_released(gbc::input::Keycode::Down);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                        gbc.mem.input.key_pressed(gbc::input::Keycode::Up);
                    },
                    Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                        gbc.mem.input.key_released(gbc::input::Keycode::Up);
                    },
                    Event::MouseButtonDown { .. } => {
                        println!("PC: {:04X} op {:02X}", gbc.regs.pc, gbc.mem.read(gbc.regs.pc));
                    }
                    _ => {}
                }
            }
            
            let cycles = gbc.step_cycles();
            gbc.mem.gpu.gpu_step(&mut display, cycles);
            gbc.mem.mmu_step(cycles);
        }
        Ok(())
    } else {
        panic!("No cartridge found");
    }
}
