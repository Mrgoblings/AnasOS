use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::geometry::{Dimensions, Size};
use embedded_graphics::primitives::Rectangle;
use spin::Mutex;
use core::sync::atomic::{AtomicBool, Ordering};

pub mod mapping;

pub static mut FRAMEBUFFER: Mutex<Option<Framebuffer>> = Mutex::new(None);


pub struct Framebuffer<'a> {
    width: usize,
    height: usize,
    front_buffer: &'a mut [Rgb888],
    back_buffer: &'a mut [Rgb888],
    swap_requested: AtomicBool,
}

impl<'a> Framebuffer<'a> {
    pub fn new(width: usize, height: usize, front: &'a mut [Rgb888], back: &'a mut [Rgb888]) -> Self {
        Self {
            width,
            height,
            front_buffer: front,
            back_buffer: back,
            swap_requested: AtomicBool::new(false),
        }
    }

    pub fn swap_buffers(&mut self) {
        if self.swap_requested.load(Ordering::Relaxed) {
            core::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
            self.swap_requested.store(false, Ordering::Relaxed);
        }
    }

    pub fn request_swap(&self) {
        self.swap_requested.store(true, Ordering::Relaxed);
    }
    
    pub fn front_buffer(&self) -> &[Rgb888] {
        self.front_buffer
    }
    
    pub fn back_buffer_mut(&mut self) -> &mut [Rgb888] {
        self.back_buffer
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Rgb888) {
        let width = self.width;
        if x < self.width && y < self.height {
            self.back_buffer_mut()[y * width + x] = color;
        }
    }
}

impl<'a> Dimensions for Framebuffer<'a> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::zero(), Size::new(self.width as u32, self.height as u32))
    }
}

impl<'a> DrawTarget for Framebuffer<'a> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let width = self.width;
        for Pixel(coord, color) in pixels {
            if coord.x >= 0 && coord.y >= 0 && coord.x < self.width as i32 && coord.y < self.height as i32 {
                self.back_buffer_mut()[width * coord.y as usize + coord.x as usize] = color;
            }
        }
        self.request_swap();
        Ok(())
    }
}


pub struct FramePosition {
    pub x: usize,
    pub y: usize,
    pub color: Rgb888,
}

impl FramePosition {
    pub fn new(x: usize, y: usize, color: Rgb888) -> Self {
        Self {
            x,
            y,
            color,
        }
    }
}
