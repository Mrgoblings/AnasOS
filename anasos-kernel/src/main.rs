#![no_std]
#![no_main]

extern crate alloc;
use core::panic::PanicInfo;

use alloc::boxed::Box;
use anasos_kernel::{
    allocator, apps, framebuffer::{self, mapping::map_framebuffer, FRAMEBUFFER}, hlt, init, memory::{
        self,
        memory_map::{FrameRange, FromMemoryMapTag, MemoryMap, MemoryRegion, MemoryRegionType},
        BootInfoFrameAllocator,
    }, println, serial_println, task::{
        executor::Executor, Task,
    }
};
use embedded_graphics::pixelcolor::Rgb888;
use x86_64::{
    structures::paging::{Mapper, Page, Size4KiB},
    PhysAddr, VirtAddr,
};

// from linker
extern "C" {
    static _heap_start: u64;
    static _heap_end: u64;
}

extern crate multiboot2;
use multiboot2::{BootInformation, BootInformationHeader};



#[no_mangle]
pub extern "C" fn _start(mb_magic: u32, mbi_ptr: u32) -> ! {
    if mb_magic != multiboot2::MAGIC {
        panic!("Invalid Multiboot2 magic number");
    }

    println!("Multiboot2 magic number: {:#x}", mb_magic);
    println!("Multiboot2 info pointer: {:#x}", mbi_ptr);

    println!("Multiboot2 Header:");

    unsafe {
        let header_bytes = core::slice::from_raw_parts(mbi_ptr as *const u8, 32);
        println!("Multiboot2 Header (First 32 Bytes): {:x?}", header_bytes);
    }

    let boot_info_res = unsafe { BootInformation::load(mbi_ptr as *const BootInformationHeader) };

    let boot_info;
    match boot_info_res {
        Ok(info) => {
            boot_info = info;
        }
        Err(e) => panic!("Failed to load Multiboot2 info: {:?}", e),
    }

    let _cmd = boot_info.command_line_tag();

    if let Some(bootloader_name) = boot_info.boot_loader_name_tag() {
        println!("Bootloader: {:?}", bootloader_name.name().ok());
    }

    // Access the memory map
    if let Some(memory_map_tag) = boot_info.memory_map_tag() {
        for area in memory_map_tag.memory_areas() {
            println!(
                "Memory area: start = {:#x}, length = {:#x}, type = {:?}",
                area.start_address(),
                area.size(),
                area.typ()
            );
        }
    }

    println!("");

    // #[cfg(notest)]
    kernel_main(&boot_info);

    // #[cfg(test)]
    // test_kernel_main(&*BOOT_INFO);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panicked: \n{}", info);
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);

    hlt();
}

fn kernel_main(boot_info: &BootInformation) -> ! {
    println!("Kernel Start:");

    init();

    let phys_mem_offset = VirtAddr::new(0);
    println!("Physical memory offset: {:?}", phys_mem_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut memory_map: MemoryMap =
        MemoryMap::from_memory_map_tag(boot_info.memory_map_tag().unwrap());

    for region in memory_map.iter() {
        println!("{:?}", region);
    }

    let framebuffer_tag = boot_info
        .framebuffer_tag()
        .unwrap()
        .ok()
        .ok_or("No framebuffer tag found")
        .unwrap();

    let framebuffer_phys_addr = PhysAddr::new(framebuffer_tag.address());
    println!("Framebuffer physical address: {:?}", framebuffer_phys_addr);

    let framebuffer_start = framebuffer_tag.address() as u64;
    let framebuffer_size = framebuffer_tag.pitch() as u64 * framebuffer_tag.height() as u64;
    let framebuffer_width = framebuffer_tag.width() as u64;
    let framebuffer_height = framebuffer_tag.height() as u64;

    println!("Framebuffer start: {:#x}", framebuffer_start);
    println!("Framebuffer size: {}", framebuffer_size);
    println!("Framebuffer width: {}", framebuffer_width);
    println!(
        "Framebuffer end: {:#x}",
        framebuffer_start + framebuffer_size
    );
    println!("Framebuffer height: {}", framebuffer_height);

    // reserve front framebuffer memory
    memory_map.add_region(MemoryRegion {
        range: FrameRange::new(framebuffer_start, framebuffer_start + framebuffer_size),
        region_type: MemoryRegionType::Reserved,
    });

    println!("Heap start: {:#x}", unsafe { _heap_start });
    println!("Heap end: {:#x}", unsafe { _heap_end });

    // Back Buffer find physical address
    let mut back_buffer_phys_addr: PhysAddr = PhysAddr::new(0); // it is never 0, but it is initialized to 0
    for region in memory_map.iter() {
        if region.region_type == MemoryRegionType::Usable
            && region.range.end_addr() - region.range.start_addr() >= framebuffer_size
        {
            let start = region.range.start_addr();
            let end = region.range.end_addr();

            for current in (start..end).step_by(4096) {
                if current >= unsafe { _heap_start } as u64 && current <= unsafe { _heap_end } as u64 ||
                   current + framebuffer_size >= unsafe { _heap_start } as u64 && current + framebuffer_size <= unsafe { _heap_end } as u64 {
                    continue;
                }
                let page = Page::<Size4KiB>::containing_address(VirtAddr::new(current));
            
                if mapper.translate_page(page).is_err() {
                    back_buffer_phys_addr = PhysAddr::new(current);
                    break;
                }
            
                if end - current < framebuffer_size {
                    break;
                }
            }
            
            if back_buffer_phys_addr.as_u64() != 0 {
                break;
            }
        }
    }

    if back_buffer_phys_addr.as_u64() == 0 {
        panic!("Failed to find a suitable memory region for the back buffer");
    }

    // reserve back framebuffer memory
    memory_map.add_region(MemoryRegion {
        range: FrameRange::new(back_buffer_phys_addr.as_u64(), back_buffer_phys_addr.as_u64() + framebuffer_size),
        region_type: MemoryRegionType::Reserved,
    });

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&mut memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");
    println!("Heap initialized");
    // VALID HEAP ALLOCATION STARTS HERE

    println!("Front framebuffer physical address: {:?}", framebuffer_phys_addr);
    // Front Buffer allocation
    match map_framebuffer(
        framebuffer_phys_addr,
        framebuffer_size,
        VirtAddr::new(framebuffer_phys_addr.as_u64()),
        &mut mapper,
        &mut frame_allocator,
    ) {
        Ok(_) => println!("Front framebuffer mapped"),
        Err(e) => panic!("Front framebuffer mapping failed: {:?}", e),
    }

    println!("Back framebuffer physical address: {:?}", back_buffer_phys_addr);
    // Back Buffer allocation
    match map_framebuffer(
        back_buffer_phys_addr,
        framebuffer_size as u64,
        VirtAddr::new(back_buffer_phys_addr.as_u64()),
        &mut mapper,
        &mut frame_allocator,
    ) {
        Ok(_) => println!("Back framebuffer mapped"),
        Err(e) => panic!("Back framebuffer mapping failed: {:?}", e),
    }

    let framebuffer = framebuffer::Framebuffer::new(
        framebuffer_width as usize,
        framebuffer_height as usize,
        unsafe {
            core::slice::from_raw_parts_mut(
                framebuffer_phys_addr.as_u64() as *mut Rgb888,
                framebuffer_height as usize * framebuffer_width as usize,
            )
        },
        unsafe {
            core::slice::from_raw_parts_mut(
                back_buffer_phys_addr.as_u64() as *mut Rgb888,
                framebuffer_height as usize * framebuffer_width as usize,
            )
        },
    );

    //set new framebuffer
    unsafe { FRAMEBUFFER.lock().replace(framebuffer); };


    //setup the static APPS list
    let terminal = apps::terminal::Terminal::new("Terminal", "Quack-Line", 1);
    apps::add_app(Box::new(terminal));
                
    let mut executor = Executor::new();
    // executor.spawn(Task::new(keyboard::load_keypresses()));
    
    executor.spawn(Task::new(apps::apps_lifecycle()));
    executor.run(); // This function will never return
}

fn test_kernel_main(_boot_info: &BootInformation) -> ! {
    println!("Running tests");
    // test code here
    println!("Tests passed");

    hlt();
}
