#![no_std]
#![no_main]

extern crate alloc;
use core::panic::PanicInfo;

use anasos_kernel::{
    allocator, framebuffer::{self, mapping::{check_framebuffer_mapping, map_framebuffer}},
    hlt, init,
    memory::{
        self, is_identity_mapped,
        memory_map::{FrameRange, FromMemoryMapTag, MemoryMap, MemoryRegion, MemoryRegionType},
        BootInfoFrameAllocator,
    },
    println, serial_println,
    task::{executor::Executor, keyboard, Task},
};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    text::Text,
};
use x86_64::{
    PhysAddr, VirtAddr
};

extern crate multiboot2;
use multiboot2::{BootInformation, BootInformationHeader};

#[no_mangle]
pub extern "C" fn _start(mb_magic: u32, mbi_ptr: u32) -> ! {
    if mb_magic != multiboot2::MAGIC {
        panic!("Invalid Multiboot2 magic number");
    }

    let boot_info =
        unsafe { BootInformation::load(mbi_ptr as *const BootInformationHeader).unwrap() };
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
    println!("Framebuffer end: {:#x}", framebuffer_start + framebuffer_size);
    println!("Framebuffer height: {}", framebuffer_height);

    // reserve framebuffer memory
    memory_map.add_region(MemoryRegion {
        range: FrameRange::new(framebuffer_start, framebuffer_start + framebuffer_size),
        region_type: MemoryRegionType::Reserved,
    });


    // Calculate total pages usable
    let total_pages: u64 = memory_map.iter()
    .filter(|region| region.region_type == MemoryRegionType::Usable)
    .map(|region| {
        let start = region.range.start_addr();
        let end = region.range.end_addr();
        (end - start) / 4096 // 4 KiB page size
    })
    .sum();

    println!("Framebuffer total pages required: {}", total_pages);


    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&mut memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");
    println!("Heap initialized");
    // VALID HEAP ALLOCATION STARTS HERE


    map_framebuffer(
        framebuffer_phys_addr,
        framebuffer_size,
        VirtAddr::new(framebuffer_phys_addr.as_u64()),
        &mut mapper,
        &mut frame_allocator,
    )
    .expect("Framebuffer mapping failed");

    println!("Framebuffer mapped");

    check_framebuffer_mapping(&mut mapper, framebuffer_tag);
    if is_identity_mapped(VirtAddr::new(framebuffer_phys_addr.as_u64()), &mapper) {
        println!(
            "Framebuffer identity mapped to address: {:x}",
            framebuffer_phys_addr.as_u64()
        );
    } else {
        panic!("Framebuffer not identity mapped");
    }


    let mut framebuffer = framebuffer::Framebuffer::new(
        framebuffer_width as usize,
        framebuffer_height as usize,
        unsafe {
            core::slice::from_raw_parts_mut(
                0xfd000000 as *mut Rgb888,
                framebuffer_height as usize * framebuffer_width as usize
            )
        },
    );

    // Draw a circle
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::RED)
        .stroke_width(1)
        .fill_color(Rgb888::GREEN)
        .build();

    Circle::new(Point::new(100, 100), 50)
        .into_styled(style)
        .draw(&mut framebuffer)
        .unwrap();

    // Draw text
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X9)
        .text_color(Rgb888::WHITE)
        .build();

    Text::new("Hello, OS!", Point::new(10, 10), text_style)
        .draw(&mut framebuffer)
        .unwrap();


    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run(); // This function will never return
}

fn test_kernel_main(_boot_info: &BootInformation) -> ! {
    println!("Running tests");
    // test code here
    println!("Tests passed");

    hlt();
}

// Test async functions
async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

use core::ptr;

pub fn write_to_mmio(addr: u64, value: u32) {
    unsafe {
        ptr::write_volatile(addr as *mut u32, value);
    }
}

pub fn read_from_mmio(addr: u64) -> u32 {
    unsafe { ptr::read_volatile(addr as *const u32) }
}
