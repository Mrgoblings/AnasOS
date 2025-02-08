use embedded_graphics::{mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder}, pixelcolor::Rgb888, prelude::*, primitives::{Circle, PrimitiveStyleBuilder}, text::Text};

use crate::framebuffer::FRAMEBUFFER;

pub fn draw() {
    // Draw a circle
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::RED)
        .stroke_width(1)
        .fill_color(Rgb888::BLUE)
        .build();

    // Draw text
    let text_style = MonoTextStyleBuilder::new()
    .font(&FONT_10X20)
    .text_color(Rgb888::WHITE)
    .build();

    unsafe {
        let mut framebuffer = FRAMEBUFFER.lock();
        let framebuffer = framebuffer.as_mut().expect("framebuffer lock poisoned");

        Circle::new(Point::new(100, 100), 50)
            .into_styled(style)
            .draw(framebuffer)
            .unwrap();

        Text::new("Hello, OS!", Point::new(10, 20), text_style)
            .draw(framebuffer)
            .unwrap();
    };
}