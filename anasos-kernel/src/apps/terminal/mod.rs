use core::{future::Future, pin::Pin, str};

use alloc::{boxed::Box, string::String};
use crossbeam_queue::ArrayQueue;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    text::Text,
};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

use crate::{framebuffer::FRAMEBUFFER, println};

use super::App;

const BUFFER_SIZE: usize = 1000;

pub struct Terminal {
    //main, 
    name: &'static str,
    priority: u8,
    title: &'static str,

    //input queues
    scancode_queue: ArrayQueue<u8>,

    // buffer functionallity here
    buffer: [char; BUFFER_SIZE],
    keyboard: Keyboard<layouts::Us104Key, ScancodeSet1>,
    cursor: usize,
}

impl Terminal {
    pub fn new(name: &'static str, title: &'static str, priority: u8) -> Self {
        Terminal {
            name,
            priority,
            title,
            scancode_queue: ArrayQueue::new(100),
            buffer: ['\0'; BUFFER_SIZE],
            keyboard: Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore,
            ),
            cursor: 0,
        }
    }
}

impl App for Terminal {
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
        println!("Terminal add_key_input: scancode: {}", scancode);
        if let Err(_) = self.scancode_queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
            return Err(());
        }
        Ok(())
    }


    // lifecycle methods
    fn init(&self) {
        println!("Initializing {}", self.name);
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

        let title_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(Rgb888::CSS_GRAY)
            .build();

        println!("Setting up text style");

        {
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
            if text.len() == 0 {
                text = String::from("Type something");
            }
            Text::new(&self.title, Point::new(600, 20), title_style)
                .draw(framebuffer)
                .unwrap();
            Text::new(&text, Point::new(60, 20), text_style)
                .draw(framebuffer)
                .unwrap();
        }
        println!("Drew text");
    }

    fn update(&mut self) {
        println!("Updating {}", self.name);

        //keyboard input
        if let Some(scancode) = self.scancode_queue.pop() {
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
                        }
                    }
                }
            }
        }
    }
}
