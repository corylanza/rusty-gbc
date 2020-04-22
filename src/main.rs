use std::env;
mod gbc;
mod debugger;

use gbc::Cpu;
use debugger::Debugger;
use gbc::gpu::Gpu;

extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // let tiles_window = video_subsystem.window("Tileset", 16 * 8 * SCALE, (384 / 16) * 8 * SCALE)
        //     .position_centered()
        //     .opengl()
        //     .resizable()
        //     .build()
        //     .unwrap();
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

        let gpu = Gpu::new(canvas).unwrap();
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
                    _ => {}
                }
            }
            
            gbc.cpu_step();
        }
        Ok(())
    } else {
        panic!("No cartridge found");
    }
}
