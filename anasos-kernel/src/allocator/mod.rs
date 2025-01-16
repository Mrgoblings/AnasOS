use fixed_size_block::FixedSizeBlockAllocator;
use multiboot2::FramebufferTag;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

pub mod fixed_size_block;

unsafe extern "C" {
    unsafe static heap_bottom: usize;
    unsafe static heap_top: usize;
}

fn heap_start() -> usize {
    unsafe { &heap_bottom as *const _ as usize }
}

fn heap_size() -> usize {
    unsafe { &heap_top as *const _ as usize - &heap_bottom as *const _ as usize }
}

#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(heap_start() as u64);
        let heap_end = heap_start + heap_size() - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    // setup the global heap allocator
    unsafe {
        ALLOCATOR.lock().init(heap_start(), heap_size());
    }

    Ok(())
}

// pub fn init_framebuffer(
//     framebuffer: &FramebufferTag,
//     mapper: &mut impl Mapper<Size4KiB>,
//     frame_allocator: &mut impl FrameAllocator<Size4KiB>,
// ) -> Result<(), MapToError<Size4KiB>> {
//     let framebuffer_start = VirtAddr::new(framebuffer.address() as u64);
//     let framebuffer_end = framebuffer_start + (framebuffer.pitch() * framebuffer.height() as u32) as u64;
    
//     let page_range = {
//         let framebuffer_start_page = Page::containing_address(framebuffer_start);
//         let framebuffer_end_page = Page::containing_address(framebuffer_end - 1u64);
//         Page::range_inclusive(framebuffer_start_page, framebuffer_end_page)
//     };

//     let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
//     for page in page_range {
//         let frame = frame_allocator
//             .allocate_frame()
//             .ok_or(MapToError::FrameAllocationFailed)?;
//         unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
//     }

//     // initialize new framebuffer allocator that will alocate the framebuffer memory only once and it will be mapped.
//     let mut framebuffer_allocator: FixedSizeBlockAllocator = FixedSizeBlockAllocator::new();
//     unsafe {
//         framebuffer_allocator.init(framebuffer_start.as_u64() as usize, framebuffer_end.as_u64() as usize);
//     }


//     Ok(())
// }

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}
