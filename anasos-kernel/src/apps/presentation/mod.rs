use core::str;

use alloc::{format, string::String, sync::Arc, vec::Vec};
use crossbeam_queue::ArrayQueue;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    text::Text,
};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

use crate::{framebuffer::{ FRAMEBUFFER, Framebuffer }, println};

use super::App;

mod data;

use data::SLIDES;

pub const BUFFER_SIZE: usize = 1000;

pub struct Presentation {
    name: &'static str,
    priority: u8,
    title: &'static str,
    scancode_queue: Arc<ArrayQueue<u8>>,
    keyboard: Keyboard<layouts::Us104Key, ScancodeSet1>,

    index: usize,
}

impl Presentation {
    pub fn new(name: &'static str, title: &'static str, priority: u8) -> Self {
        Presentation {
            name,
            priority,
            title,
            scancode_queue: Arc::new(ArrayQueue::new(100)),
            keyboard: Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore,
            ),
            index: 0,
        }
    }

    fn next_slide(&mut self) {
        if self.index < SLIDES.len() - 1 {
            self.index += 1;
        }
    }

    fn previous_slide(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    fn handle_input(&mut self) {
        while let Some(scancode) = self.scancode_queue.pop() {
            if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
                if let Some(key) = self.keyboard.process_keyevent(key_event) {
                    match key {
                        DecodedKey::RawKey(key_code) => {
                            match key_code {
                                pc_keyboard::KeyCode::Backspace | pc_keyboard::KeyCode::Tab => {
                                    // never reached, unicode version handles them
                                    return;
                                }
                                pc_keyboard::KeyCode::F1
                                | pc_keyboard::KeyCode::F2
                                | pc_keyboard::KeyCode::F3
                                | pc_keyboard::KeyCode::F4
                                | pc_keyboard::KeyCode::F5
                                | pc_keyboard::KeyCode::F6
                                | pc_keyboard::KeyCode::F7
                                | pc_keyboard::KeyCode::F8
                                | pc_keyboard::KeyCode::F9
                                | pc_keyboard::KeyCode::F10
                                | pc_keyboard::KeyCode::F11
                                | pc_keyboard::KeyCode::F12 => {
                                    return;
                                }
                                pc_keyboard::KeyCode::ArrowDown => {
                                    return;
                                }
                                pc_keyboard::KeyCode::ArrowUp => {
                                    return;
                                }
                                pc_keyboard::KeyCode::ArrowLeft => {
                                    self.previous_slide();
                                    return;
                                }
                                pc_keyboard::KeyCode::ArrowRight
                                | pc_keyboard::KeyCode::Spacebar => {
                                    self.next_slide();
                                    return;
                                }
                                _ => {
                                    return;
                                }
                            }
                        }

                        DecodedKey::Unicode(character) => match character {
                            ' ' => {
                                self.next_slide();
                                return;
                            }
                            _ => {
                                return;
                            }
                        },
                    }
                }
            }
        }
        return;
    }

    /// Draw a single P6 PPM image at (0,0) in the frame-buffer.
    ///
    /// Expects `data` = b"P6\n<width> <height>\n<max>\n<binary-RGB…>".
    fn draw_ppm(data: &[u8], framebuffer: &mut Framebuffer) {
        // 1) Find end of the third newline (i.e. end of the header)
        let header_end = data
            .iter()
            .enumerate()
            .filter(|&(_, &b)| b == b'\n')
            .map(|(i, _)| i)
            .nth(2)
            .expect("PPM header incomplete")
            + 1;

        // 2) Parse ASCII header
        let header = str::from_utf8(&data[..header_end]).expect("PPM header not valid UTF-8");
        let mut parts = header.split_whitespace();
        let magic = parts.next().unwrap();
        assert_eq!(magic, "P6", "only raw P6 supported");
        let width: usize = parts.next().unwrap().parse().unwrap();
        let height: usize = parts.next().unwrap().parse().unwrap();
        let _max: usize = parts.next().unwrap().parse().unwrap();

        // 3) Pixel bytes follow
        let pixels = &data[header_end..];
        assert_eq!(pixels.len(), width * height * 3, "pixel count mismatch");

        // 5) Blit: each pixel is 3 bytes R,G,B → 0xRRGGBB u32
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) * 3;
                let r = pixels[idx] as u32;
                let g = pixels[idx + 1] as u32;
                let b = pixels[idx + 2] as u32;
                // let color = (r << 16) | (g << 8) | b;
                let color = Rgb888::new(r,g,b);
                framebuffer.draw_pixel(x as u32, y as u32, color);
            }
        }
    }
}

impl App for Presentation {
    //getters
    fn name(&self) -> &'static str {
        self.name
    }

    fn priority(&self) -> u8 {
        self.priority
    }

    fn title(&self) -> &'static str {
        self.title
    }

    //input methods
    fn scancode_push(&self, scancode: u8) -> Result<(), ()> {
        self.scancode_queue.push(scancode).map_err(|_| ())
    }

    // lifecycle methods
    fn init(&mut self) {
        self.log(&format!("Initializing {}", self.name));

        // self.slides
        //     .add_slide(Slide::new(String::from("Hello World")));
        // self.slides
        //    .add_slide(Slide::new(String::from("Hello World 2")));
    }

    unsafe fn draw(&mut self) {
        self.log("Drawing terminal");

        // let style = PrimitiveStyleBuilder::new()
        //     .stroke_color(Rgb888::RED)
        //     .stroke_width(1)
        //     .fill_color(Rgb888::BLUE)
        //     .build();

        // self.log("Setting up style");

        // // Draw text
        // let text_style = MonoTextStyleBuilder::new()
        //     .font(&FONT_10X20)
        //     .text_color(Rgb888::WHITE)
        //     .build();

        // let title_style = MonoTextStyleBuilder::new()
        //     .font(&FONT_10X20)
        //     .text_color(Rgb888::CSS_GRAY)
        //     .build();

        // self.log("Setting up text style");

        {
            let mut framebuffer = FRAMEBUFFER.lock();
            self.log("Got framebuffer lock");
            let framebuffer = framebuffer.as_mut().expect("framebuffer lock poisoned");
            self.log("Got framebuffer");

            framebuffer.clear(Rgb888::RED).unwrap();

            self.draw_ppm(
                SLIDES[self.index].data,
                framebuffer,
            );
        }
        self.log("Drew slide");
    }

    fn update(&mut self) {
        self.log(&format!("Updating {}", self.name));
        self.handle_input();
    }

    fn log(&self, message: &str) {
        println!("Presentation> {}", message);
    }
}

unsafe impl Sync for Presentation {}
unsafe impl Send for Presentation {}
