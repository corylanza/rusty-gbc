extern crate stdweb;

mod canvas;

use canvas::Canvas;
use rusty_gbc::gbc::Cpu;
//use debugger::Debugger;
use rusty_gbc::{Display};
use rusty_gbc::gbc::gpu::Gpu;
//use std::boxed::Box;
use stdweb::js;
use stdweb::web::FileReader;
use stdweb::web::FileReaderResult;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::web::{document, event::KeyDownEvent, event::KeyUpEvent, IEventTarget};

fn run_rom(file_reader: FileReader) {
    match file_reader.result() {
        Some(value) => match value {
            FileReaderResult::ArrayBuffer(array) => {
                let bytes = Vec::<u8>::from(array);
                let title = String::from_utf8(bytes[0x0134..0x0143].to_vec()).unwrap();
                let title_elem = document()
                    .query_selector("#game-title").unwrap();
                match title_elem {
                    Some(elem) => { js! {
                        @{elem}.innerText = @{title.clone()}
                    }},
                    None => {}
                };
                let canvas = Canvas::new("#canvas");
                
                let color_mode = bytes[0x143] & 0x80 == 0x80 || bytes[0x143] & 0xC0 == 0xC0;
                let gpu = Gpu::new(color_mode).unwrap();
                let gbc = Rc::new(RefCell::new(Cpu::new(bytes, gpu)));

                stdweb::web::document().add_event_listener({
                    let gbc = gbc.clone();
                    move |event: KeyDownEvent| {
                        match event.key().as_ref() {
                            "ArrowLeft" => gbc.borrow_mut().mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::Left),
                            "ArrowRight" => gbc.borrow_mut().mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::Right),
                            "ArrowDown" => gbc.borrow_mut().mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::Down),
                            "ArrowUp" => gbc.borrow_mut().mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::Up),
                            "Enter" => gbc.borrow_mut().mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::Start),
                            " " => gbc.borrow_mut().mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::Select),
                            "s" => gbc.borrow_mut().mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::A),
                            "a" => gbc.borrow_mut().mem.input.key_pressed(rusty_gbc::gbc::input::Keycode::B),
                            _ => {}
                        };
                    }
                });

                stdweb::web::document().add_event_listener({
                    let gbc = gbc.clone();
                    move |event: KeyUpEvent| {
                        match event.key().as_ref() {
                            "ArrowLeft" => gbc.borrow_mut().mem.input.key_released(rusty_gbc::gbc::input::Keycode::Left),
                            "ArrowRight" => gbc.borrow_mut().mem.input.key_released(rusty_gbc::gbc::input::Keycode::Right),
                            "ArrowDown" => gbc.borrow_mut().mem.input.key_released(rusty_gbc::gbc::input::Keycode::Down),
                            "ArrowUp" => gbc.borrow_mut().mem.input.key_released(rusty_gbc::gbc::input::Keycode::Up),
                            "Enter" => gbc.borrow_mut().mem.input.key_released(rusty_gbc::gbc::input::Keycode::Start),
                            " " => gbc.borrow_mut().mem.input.key_released(rusty_gbc::gbc::input::Keycode::Select),
                            "s" => gbc.borrow_mut().mem.input.key_released(rusty_gbc::gbc::input::Keycode::A),
                            "a" => gbc.borrow_mut().mem.input.key_released(rusty_gbc::gbc::input::Keycode::B),
                            _ => {}
                        };
                    }
                });

                game_loop(gbc, Rc::new(RefCell::new(canvas)), 1);
            },
            _ => {},
        }
        None => {}
    }
}

fn game_loop(gbc: Rc<RefCell<Cpu>>, canvas: Rc<RefCell<dyn Display>>, time: u32) {
    stdweb::web::set_timeout(
        move || {
            game_loop(gbc.clone(), canvas.clone(), time);
            gbc.borrow_mut().run_one_frame(&mut *canvas.borrow_mut());
        },
        time,
    );
}

fn main() {
    stdweb::initialize();
    js! {
        window.reader = {};
        window.reader.run_rom = @{run_rom};
    }
    stdweb::event_loop();
}