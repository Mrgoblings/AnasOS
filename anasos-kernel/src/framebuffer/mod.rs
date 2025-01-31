use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::geometry::{Dimensions, Size};
use embedded_graphics::primitives::Rectangle;

pub mod mapping;

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: &'static mut [Rgb888],
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, buffer: &'static mut [Rgb888]) -> Self {
        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn buffer(&self) -> &[Rgb888] {
        self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut [Rgb888] {
        self.buffer
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Dimensions for Framebuffer {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::zero(), Size::new(self.width as u32, self.height as u32))
    }
}

impl DrawTarget for Framebuffer {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x >= 0 && coord.y >= 0 && coord.x < self.width as i32 && coord.y < self.height as i32 {
                self.buffer[self.width() * coord.y as usize + coord.x as usize] = color;
            }
        }
        Ok(())
    }
}
