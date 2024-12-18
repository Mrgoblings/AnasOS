//FROM https://github.com/rust-osdev/bootloader/blob/v0.9.25/src/boot_info.rs#L7
use core::slice;

use memory_map::{MemoryMap, MemoryRegion, MemoryRegionType, E820MemoryRegion};
use traits::BootInfo;
use usize_conversions::usize_from;
use fixedvec::FixedVec;

use x86_64::structures::paging::{
    frame::PhysFrameRange, page_table::PageTableEntry, Mapper, Page, PageTable, PageTableFlags,
    PageTableIndex, PhysFrame, RecursivePageTable, Size2MiB, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};


pub mod memory_map;
pub mod traits;
pub mod level4_entries;
pub mod page_table;

pub fn create_from(memory_map_addr: VirtAddr, entry_count: u64) -> MemoryMap {
    let memory_map_start_ptr = usize_from(memory_map_addr.as_u64()) as *const E820MemoryRegion;
    let e820_memory_map =
        unsafe { slice::from_raw_parts(memory_map_start_ptr, usize_from(entry_count)) };

    let mut memory_map = MemoryMap::new();
    for region in e820_memory_map {
        memory_map.add_region(MemoryRegion::from(*region));
    }

    memory_map.sort();

    let mut iter = memory_map.iter_mut().peekable();
    while let Some(region) = iter.next() {
        if let Some(next) = iter.peek() {
            if region.range.end_frame_number > next.range.start_frame_number
                && region.region_type == MemoryRegionType::Usable
            {
                region.range.end_frame_number = next.range.start_frame_number;
            }
        }
    }

    memory_map
}


pub struct IdentityMappedAddr(PhysAddr);

impl IdentityMappedAddr {
    fn phys(&self) -> PhysAddr {
        self.0
    }

    fn virt(&self) -> VirtAddr {
        VirtAddr::new(self.0.as_u64())
    }

    fn as_u64(&self) -> u64 {
        self.0.as_u64()
    }
}

// The bootloader_config.rs file contains some configuration constants set by the build script:
// PHYSICAL_MEMORY_OFFSET: The offset into the virtual address space where the physical memory
// is mapped if the `map_physical_memory` feature is activated.
//
// KERNEL_STACK_ADDRESS: The virtual address of the kernel stack.
//
// KERNEL_STACK_SIZE: The number of pages in the kernel stack.
include!(concat!(env!("OUT_DIR"), "/bootloader_config.rs"));

unsafe extern "C" {
    unsafe static mmap_ent: usize;
    unsafe static _memory_map: usize;
    unsafe static _kernel_start_addr: usize;
    unsafe static _kernel_end_addr: usize;
    unsafe static _kernel_size: usize;
    unsafe static __page_table_start: usize;
    unsafe static __page_table_end: usize;
    unsafe static __bootloader_end: usize;
    unsafe static __bootloader_start: usize;
    unsafe static _p4: usize;
}


pub unsafe fn get() -> BootInfo {

    let mut segments = FixedVec::new(&mut preallocated_space);

    // Mark used virtual addresses
    let mut level4_entries: level4_entries::UsedLevel4Entries = level4_entries::UsedLevel4Entries::new(&segments);


    let kernel_start = IdentityMappedAddr(PhysAddr::new(0x400000));
    let memory_map_addr = &_memory_map as *const _ as u64;
    let memory_map_entry_count = (mmap_ent & 0xff) as u64; // Extract lower 8 bits
    let page_table_start = &__page_table_start as *const _ as u64;
    let page_table_end = &__page_table_end as *const _ as u64;
    let bootloader_start = &__bootloader_start as *const _ as u64;
    let bootloader_end = &__bootloader_end as *const _ as u64;
    let p4_physical = &_p4 as *const _ as u64;
    
    let memory_map: MemoryMap = create_from(VirtAddr::new(memory_map_addr), memory_map_entry_count);

    let max_phys_addr: u64 = memory_map
        .iter()
        .map(|r| r.range.end_addr())
        .max()
        .expect("no physical memory regions found");


    
    let recursive_index =
    PageTableIndex::new(level4_entries.get_free_entries(1).try_into().unwrap());
    let recursive_page_table_addr = Page::from_page_table_indices(
        recursive_index,
        recursive_index,
        recursive_index,
        recursive_index,
    )
    .start_address();
    let page_table = unsafe { &mut *(recursive_page_table_addr.as_mut_ptr()) };
    let mut rec_page_table: RecursivePageTable<'_> =
        RecursivePageTable::new(page_table).expect("recursive page table creation failed");




    // Map a page for the boot info structure
    let boot_info_page = get_boot_info_page(level4_entries, rec_page_table);
    // If no kernel stack address is provided, map the kernel stack after the boot info page
    let kernel_stack_address = match KERNEL_STACK_ADDRESS {
        Some(addr) => Page::containing_address(VirtAddr::new(addr)),
        None => boot_info_page + 1,
    };
    // Map kernel segments.
    let kernel_memory_info = page_table::map_kernel(
        kernel_start.phys(),
        kernel_stack_address,
        KERNEL_STACK_SIZE,
        &segments,
        &mut rec_page_table,
        &mut frame_allocator,
    )
    .expect("kernel mapping failed");





    let physical_memory_offset = get_physical_memory_offset(level4_entries, rec_page_table, max_phys_addr);

    BootInfo::new(memory_map, kernel_memory_info.tls_segment, recursive_page_table_addr.as_u64(), physical_memory_offset)
}



fn get_boot_info_page(mut level4_entries: level4_entries::UsedLevel4Entries, mut rec_page_table: RecursivePageTable<'_>) -> Page {
    let page: Page = match BOOT_INFO_ADDRESS {
        Some(addr) => Page::containing_address(VirtAddr::new(addr)),
        None => Page::from_page_table_indices(
            level4_entries.get_free_entries(1),
            PageTableIndex::new(0),
            PageTableIndex::new(0),
            PageTableIndex::new(0),
        ),
    };
    let frame = frame_allocator
        .allocate_frame(MemoryRegionType::BootInfo)
        .expect("frame allocation failed");
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    unsafe {
        page_table::map_page(
            page,
            frame,
            flags,
            &mut rec_page_table,
            &mut frame_allocator,
        )
    }
    .expect("Mapping of bootinfo page failed")
    .flush();
    page
}


fn get_physical_memory_offset(mut level4_entries: level4_entries::UsedLevel4Entries, mut rec_page_table: RecursivePageTable<'_>, max_phys_addr: u64) -> u64 {
    let physical_memory_offset = PHYSICAL_MEMORY_OFFSET.unwrap_or_else(|| {
        const LEVEL_4_SIZE: u64 = 4096 * 512 * 512 * 512;
        let level_4_entries = (max_phys_addr + (LEVEL_4_SIZE - 1)) / LEVEL_4_SIZE;
        Page::from_page_table_indices_1gib(
            level4_entries.get_free_entries(level_4_entries),
            PageTableIndex::new(0),
        )
        .start_address()
        .as_u64()
    });

    let virt_for_phys =
        |phys: PhysAddr| -> VirtAddr { VirtAddr::new(phys.as_u64() + physical_memory_offset) };

    let start_frame = PhysFrame::<Size2MiB>::containing_address(PhysAddr::new(0));
    let end_frame = PhysFrame::<Size2MiB>::containing_address(PhysAddr::new(max_phys_addr));

    for frame in PhysFrame::range_inclusive(start_frame, end_frame) {
        let page = Page::containing_address(virt_for_phys(frame.start_address()));
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            page_table::map_page(
                page,
                frame,
                flags,
                &mut rec_page_table,
                &mut frame_allocator,
            )
        }
        .expect("Mapping of bootinfo page failed")
        .flush();
    }

    physical_memory_offset
}