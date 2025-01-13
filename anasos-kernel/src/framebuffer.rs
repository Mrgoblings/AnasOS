use multiboot2::FramebufferTag;
use x86_64::{structures::paging::{Mapper, OffsetPageTable, Page, PhysFrame, Size4KiB}, PhysAddr, VirtAddr};
use x86_64::structures::paging::PageTableFlags;

use crate::memory::BootInfoFrameAllocator;


pub fn init() {

}

pub fn map_framebuffer_page_table(
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut BootInfoFrameAllocator,
    framebuffer_tag: &FramebufferTag,
) {
    let framebuffer_start = framebuffer_tag.address() as u64;
    let framebuffer_end = framebuffer_start + (framebuffer_tag.width() * framebuffer_tag.height() * framebuffer_tag.bpp() as u32) as u64;

    let frame_start_page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(framebuffer_start));
    let frame_end_page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(framebuffer_end));

    for page in Page::range(frame_start_page, frame_end_page) {
        let frame = PhysFrame::containing_address(PhysAddr::new(page.start_address().as_u64()));
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        let map_to_result = unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)
        };
        map_to_result.expect("Frame allocation failed. A function call on 'map_framebuffer_page_table' raised the error. failed").flush();
    }
}