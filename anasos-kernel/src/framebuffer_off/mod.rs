use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::geometry::{Dimensions, Size};
use embedded_graphics::primitives::Rectangle;

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: &'static mut [u8],
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, buffer: &'static mut [u8]) -> Self {
        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn buffer(&self) -> &[u8] {
        self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
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
        for Pixel(coord, color) in pixels {
            let x = coord.x;
            let y = coord.y;
    
            if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
                let index = ((y as usize * self.width + x as usize) * 4) as usize;
                self.buffer[index..index + 3].copy_from_slice(&[
                    color.r(),
                    color.g(),
                    color.b(),
                ]);
            }
        }
        Ok(())
    }
}
