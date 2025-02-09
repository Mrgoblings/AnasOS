use core::{future::Future, pin::Pin, str};

use alloc::{boxed::Box, string::String};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    text::Text,
};
use futures_util::{task::AtomicWaker, StreamExt};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

use crate::{framebuffer::FRAMEBUFFER, println};

use super::{App, KeyStream};

const BUFFER_SIZE: usize = 1000;
pub struct Terminal {
    name: &'static str,
    priority: u8,
    key_inputs: KeyStream,
    // waker: AtomicWaker,
    keyboard: Keyboard<layouts::Us104Key, ScancodeSet1>,
    buffer: [char; BUFFER_SIZE],
    cursor: usize,
}

impl Terminal {
    pub fn new(name: &'static str, priority: u8) -> Self {
        Terminal {
            name,
            priority,
            key_inputs: KeyStream::new(),
            // waker: AtomicWaker::new(),
            keyboard: Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore,
            ),
            buffer: ['\0'; BUFFER_SIZE],
            cursor: 0,
        }
    }
}

impl App for Terminal {
    fn name(&self) -> &'static str {
        self.name
    }

    fn priority(&self) -> u8 {
        self.priority
    }

    fn init(&self) {
        println!("Initializing terminal");
    }

    unsafe fn draw(&self) {
        println!("Drawing terminal");
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::RED)
            .stroke_width(1)
            .fill_color(Rgb888::BLUE)
            .build();

        println!("Setting up style");

        // Draw text
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(Rgb888::WHITE)
            .build();

        println!("Setting up text style");

        let mut framebuffer = FRAMEBUFFER.lock();
        println!("Got framebuffer lock");
        let framebuffer = framebuffer.as_mut().expect("framebuffer lock poisoned");
        println!("Got framebuffer");


        Circle::new(Point::new(100, 100), 50)
            .into_styled(style)
            .draw(framebuffer)
            .unwrap();

        println!("Drew circle");

        let mut text: String = self.buffer.iter().collect();
        if text.len() > 0 {
            text = String::from("Type something");
        }
        Text::new(&text, Point::new(10, 20), text_style)
            .draw(framebuffer)
            .unwrap();
        println!("Drew text");
    }

    fn load(&mut self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            self.init();
            unsafe{ self.draw() };
            println!("loading terminal after first initial draw");

            while let Some(scancode) = self.key_inputs.next().await {
                println!("Got scancode: {}", scancode);
                if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
                    if let Some(key) = self.keyboard.process_keyevent(key_event) {
                        match key {
                            DecodedKey::Unicode(character) => {
                                println!("CHAR: {}", character);
                                self.buffer[self.cursor] = character;
                                self.cursor += 1;
                            }
                            DecodedKey::RawKey(key) => {
                                println!("KEY: {:?}", key);
                                // self.buffer[self.cursor] = key;
                                // self.cursor += 1;
                            }
                        }
                    }
                }
                
                println!("Calling draw");
                unsafe { self.draw() };
            }
        })
    }

    fn add_key_input(&self, scancode: u8) {
        self.key_inputs.add_scancode(scancode);
    }
}
