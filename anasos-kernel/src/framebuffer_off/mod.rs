use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb888;

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: &'static mut [u8], // Assume 32-bit RGBA framebuffer
}

impl DrawTarget for Framebuffer {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            if let Ok((x, y)) = coord.try_into() {
                if x < self.width as i32 && y < self.height as i32 {
                    let index = ((y as usize * self.width + x as usize) * 4) as usize;
                    self.buffer[index..index + 3].copy_from_slice(&[
                        color.r(),
                        color.g(),
                        color.b(),
                    ]);
                }
            }
        }
        Ok(())
    }
}
