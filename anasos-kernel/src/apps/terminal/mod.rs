use core::str;

use alloc::{format, string::String};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    text::Text,
};

use crate::{framebuffer::FRAMEBUFFER, println, shell::Shell};

use super::App;

pub const BUFFER_SIZE: usize = 1000;


pub struct Terminal {
    name: &'static str,
    priority: u8,
    title: &'static str,
    shell: Shell,    
}

impl Terminal {
    pub fn new(name: &'static str, title: &'static str, priority: u8) -> Self {
        Terminal {
            name,
            priority,
            title,
            shell: Shell::new("AnasOS>"),
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
        self.shell.scancode_push(scancode)
    }

    // lifecycle methods
    fn init(&mut self) {
        self.log(&format!("Initializing {}", self.name));
    }

    unsafe fn draw(&mut self) {
        self.log("Drawing terminal");

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::RED)
            .stroke_width(1)
            .fill_color(Rgb888::BLUE)
            .build();

        self.log("Setting up style");
        // Draw text
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(Rgb888::WHITE)
            .build();

        let title_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(Rgb888::CSS_GRAY)
            .build();

        self.log("Setting up text style");

        {
            let mut framebuffer = FRAMEBUFFER.lock();
            self.log("Got framebuffer lock");
            let framebuffer = framebuffer.as_mut().expect("framebuffer lock poisoned");
            self.log("Got framebuffer");
            
            Text::new(&self.title, Point::new(600, 20), title_style)
                .draw(framebuffer)
                .unwrap();
            Circle::new(Point::new(800, 100), 50)
                .into_styled(style)
                .draw(framebuffer)
                .unwrap();

            self.log("Drew circle");

            let mut text: String = self.shell.get_printable(); 
            if text.len() == 0 {
                text = String::from("TERMINAL> Type something");
            }
            Text::new(&text, Point::new(60, 60), text_style)
                .draw(framebuffer)
                .unwrap();
        }
        self.log("Drew text");
    }

    fn update(&mut self) {
        self.log(&format!("Updating {}", self.name));
        
        self.shell.handle_input();
    }

    fn log(&self, message: &str) {
        println!("TERMINAL> {}", message);
    }
}

unsafe impl Sync for Terminal {}
unsafe impl Send for Terminal {}
