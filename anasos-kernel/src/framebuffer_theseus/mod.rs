//FROM https://www.theseus-os.com/Theseus/doc/src/framebuffer/lib.rs.html

use alloc::vec::Vec;
use x86_64::structures::paging::{Mapper, OffsetPageTable, Page, PageSize, PageTableFlags, PhysFrame, Size4KiB};
use x86_64::{PhysAddr, VirtAddr};
use crate::println;
use crate::memory::BootInfoFrameAllocator;
use core::marker::PhantomData;
use pixel::Pixel;

pub mod color;
pub mod pixel;
pub mod shapes;

/// A framebuffer is a region of memory interpreted as a 2D array of pixels.
pub struct Framebuffer<P: Pixel + 'static> {
    width: usize,
    height: usize,
    buffer: &'static mut [P],
    _pixel_type: PhantomData<P>,
}

impl<P: Pixel> Framebuffer<P> {
    /// Creates a new framebuffer with the given dimensions and maps it to the physical memory region.
    ///
    /// # Arguments
    /// - `width`: Width of the framebuffer in pixels.
    /// - `height`: Height of the framebuffer in pixels.
    /// - `physical_address`: Physical address of the framebuffer, if any.
    /// - `mapper`: The `OffsetPageTable` used for virtual memory mappings.
    /// - `frame_allocator`: The frame allocator for managing memory.
    pub fn new(
        width: usize,
        height: usize,
        physical_address: Option<PhysAddr>,
        mapper: &mut OffsetPageTable,
        frame_allocator: &mut BootInfoFrameAllocator,
    ) -> Result<Self, &'static str> {
        let size = width * height * core::mem::size_of::<P>();
        let size_in_pages = (size + Size4KiB::SIZE as usize - 1) / Size4KiB::SIZE as usize;

        println!("\n\nCreating framebuffer with size: {} bytes, {} pages", size, size_in_pages);

        let framebuffer_start_virt = match physical_address {
            Some(phys_addr) => {
                // Map the framebuffer's physical memory to a virtual memory region.
                let mut framebuffer_pages = Vec::new();
                for i in 0..size_in_pages {
                    let phys_frame: PhysFrame<Size4KiB> = PhysFrame::containing_address(phys_addr + (i as u64 * Size4KiB::SIZE as u64));
                    let page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(phys_addr.as_u64() + (i as u64 * (Size4KiB::SIZE as u64))));

                    unsafe {
                        mapper
                            .map_to(page, phys_frame, PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE, frame_allocator)
                            .map_err(|_| "Failed to map framebuffer pages")?
                                .flush();
                    }
                    framebuffer_pages.push(page);
                }

                framebuffer_pages[0].start_address().as_u64()
            }
            None => return Err("Virtual framebuffer creation is not yet implemented"),
        };

        // Create a mutable slice to access the framebuffer.
        let buffer = unsafe {
            let ptr = framebuffer_start_virt as *mut P;
            core::slice::from_raw_parts_mut(ptr, width * height)
        };
        println!("Framebuffer virtual address: {:#x}", framebuffer_start_virt);
        println!("Framebuffer physical address: {:#x}", physical_address.unwrap().as_u64());
        // println!("value at framebuffer_start_virt: {:#x}", buffer[0]);
        println!("Moved after reading framebuffer_start_virt");

        Ok(Self {
            width,
            height,
            buffer,
            _pixel_type: PhantomData,
        })
    }

    /// Returns a mutable reference to the framebuffer memory.
    pub fn buffer_mut(&mut self) -> &mut [P] {
        self.buffer
    }

    /// Returns the width and height of the framebuffer.
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Draws a pixel at the specified coordinate, blending it with the existing pixel.
    pub fn draw_pixel(&mut self, x: usize, y: usize, pixel: P) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = pixel.blend(self.buffer[index]);
        }
    }

    /// Overwrites a pixel at the specified coordinate without blending.
    pub fn overwrite_pixel(&mut self, x: usize, y: usize, pixel: P) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = pixel;
        }
    }

    /// Fills the entire framebuffer with the given pixel value.
    pub fn fill(&mut self, pixel: P) {
        for p in self.buffer.iter_mut() {
            *p = pixel;
        }
    }
}
