use multiboot2::FramebufferTag;
use x86_64::{
    structures::paging::{
        PageTableFlags, 
        mapper::TranslateResult, 
        FrameAllocator, 
        Mapper, 
        OffsetPageTable, 
        Page, PageSize, 
        PhysFrame, 
        Size4KiB, 
        Translate
    }, 
    PhysAddr, VirtAddr
};

use crate::{memory::BootInfoFrameAllocator, println};



// pub fn map_region(
//     mapper: &mut OffsetPageTable,
//     frame_allocator: &mut BootInfoFrameAllocator,
//     start: VirtAddr,
//     end: VirtAddr,
//     flags: PageTableFlags,
// ) {
//     let start_page = Page::containing_address(start);
//     let end_page = Page::containing_address(end - 1u64);

//     for page in Page::range_inclusive(start_page, end_page) {
//         let frame = frame_allocator.allocate_frame().expect("Frame allocation failed in map_region");
//         let map_to_result = unsafe {
//             mapper.map_to(page, frame, flags, frame_allocator)
//         };

//         map_to_result
//             .expect("Frame allocation failed in map_region")
//             .flush();
//     }
// }


// pub fn map_framebuffer_page_table(
//     mapper: &mut OffsetPageTable,
//     frame_allocator: &mut BootInfoFrameAllocator,
//     framebuffer_tag: &FramebufferTag,
// ) {
//     let framebuffer_start = framebuffer_tag.address() as u64;
//     let framebuffer_end = framebuffer_start
//         + (framebuffer_tag.width() * framebuffer_tag.height() * (framebuffer_tag.bpp() as u32) / 8) as u64;

//     println!(
//         "Mapping framebuffer: {:#x} - {:#x}",
//         framebuffer_start, framebuffer_end
//     );

//     // Align framebuffer start to the page size (4 KiB)
//     let framebuffer_start_aligned = framebuffer_start & !(Size4KiB::SIZE - 1);

//     let frame_start_page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(framebuffer_start_aligned));

//     // Ensure the range includes the last page of the framebuffer.
//     // `Page::range` is exclusive of the upper bound, so adding 1 ensures
//     // that the page containing the last byte of the framebuffer is mapped.
//     let frame_end_page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(framebuffer_end - 1)) + 1;

//     println!(
//         "framebuffer_start_aligned: {:#x}, frame_start_page: {:#x}, frame_end_page: {:#x}",
//         framebuffer_start_aligned,
//         frame_start_page.start_address().as_u64(),
//         frame_end_page.start_address().as_u64()
//     );

//     for page in Page::range(frame_start_page, frame_end_page) {
//         // Calculate the corresponding physical frame address
//         let frame_address = framebuffer_start_aligned
//             + (page.start_address().as_u64() - frame_start_page.start_address().as_u64());
//         let frame = PhysFrame::containing_address(PhysAddr::new(frame_address));

//         let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE;

//         // Unsafe because we assume the caller ensures the memory map's correctness
//         let map_to_result = unsafe {
//             mapper.map_to(page, frame, flags, frame_allocator)
//         };

//         map_to_result
//             .expect("Frame allocation failed in map_framebuffer_page_table")
//             .flush();
//     }
// }


pub fn map_framebuffer(
    framebuffer_phys_addr: PhysAddr,
    framebuffer_size: u64,
    framebuffer_virt_addr: VirtAddr,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut BootInfoFrameAllocator,
) -> Result<(), &'static str> {
    let framebuffer_start_page: Page<Size4KiB> = Page::containing_address(framebuffer_virt_addr);
    let framebuffer_end_page: Page<Size4KiB> = Page::containing_address(
        framebuffer_virt_addr + framebuffer_size as u64 - 1u64
    );

    let mut current_page = framebuffer_start_page;

    while current_page <= framebuffer_end_page {
        let frame = PhysFrame::containing_address(framebuffer_phys_addr + (current_page.start_address().as_u64() - framebuffer_virt_addr.as_u64()));
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe {
            mapper.map_to(current_page, frame, flags, frame_allocator)
                  .expect("Frame allocation failed in map_framebuffer")
                  .flush();
        }

        current_page += 1;
    }

    Ok(())
}



pub fn check_framebuffer_mapping(mapper: &impl Translate, framebuffer_tag: &FramebufferTag) {
    let framebuffer_start = framebuffer_tag.address() as u64;
    let framebuffer_end = framebuffer_start
        + (framebuffer_tag.width() * framebuffer_tag.height() * (framebuffer_tag.bpp() as u32) / 8) as u64;

    println!(
        "Checking framebuffer mapping: {:#x} - {:#x}",
        framebuffer_start, framebuffer_end
    );

    let mut all_mapped = true;
    let mut all_flags_correct = true;

    for address in (framebuffer_start..framebuffer_end).step_by(Size4KiB::SIZE as usize) {
        let virt_addr = VirtAddr::new(address);
        
        if let Some(frame_info) = mapper.translate(virt_addr).into() {
            match frame_info {
                TranslateResult::Mapped { frame, flags, offset: _ } => {
                    if !(flags.contains(x86_64::structures::paging::PageTableFlags::PRESENT)
                        && flags.contains(x86_64::structures::paging::PageTableFlags::WRITABLE))
                    {
                        println!(
                            "Incorrect flags for Virtual {:#x} -> Physical {:#x}: {:?}",
                            address,
                            frame.start_address().as_u64(),
                            flags
                        );
                        all_flags_correct = false;
                    }
                }
                _ => {
                    println!("Unmapped or invalid frame address for Virtual {:#x}", address);
                    all_mapped = false;
                }
            }
        } else {
            println!("Unmapped: Virtual {:#x}", address);
            all_mapped = false;
        }
    }

    if all_mapped && all_flags_correct {
        println!("All framebuffer addresses are mapped correctly with proper flags.");
    } else if !all_mapped {
        println!("Some framebuffer addresses are not mapped.");
    } else {
        println!("Some framebuffer pages have incorrect flags.");
    }
}
