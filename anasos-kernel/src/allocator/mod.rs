use fixed_size_block::FixedSizeBlockAllocator;
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
        // let heap_start_page = Page::containing_address(heap_start);
        // let heap_end_page = Page::containing_address(heap_end);
        
        // Changed the two lines below due to the unsafe line I had to comment further down
        // Otherwise the compiler would not be able to properly infer the type of the variables
        let heap_start_page = Page::<Size4KiB>::containing_address(heap_start);
        let heap_end_page = Page::<Size4KiB>::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        unsafe {
            match mapper.map_to(page, frame, flags, frame_allocator) {
                Ok(mf) => mf.flush(),
                Err(MapToError::PageAlreadyMapped(_)) => (),
                Err(e) => panic!("Error mapping heap page: {:?}", e),
            }
        }
    }

    // setup the global heap allocator
    unsafe {
        ALLOCATOR.lock().init(heap_start(), heap_size());
    }

    Ok(())
}

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
