#![no_std]
#![no_main]

extern crate alloc;
use core::panic::PanicInfo;

use anasos_kernel::{
    allocator, framebuffer, hlt, init, memory::{
        self,
        memory_map::{FromMemoryMapTag, MemoryMap},
        BootInfoFrameAllocator,
    }, println, serial_println, task::{executor::Executor, keyboard, Task}
};
use x86_64::{structures::paging::{frame, Mapper, Page, PageTableFlags, Size4KiB}, VirtAddr};

use x86_64::{
    structures::paging::{PageTable, PhysFrame, Translate},
    PhysAddr,
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
        println!("Bootloader: {:?}", bootloader_name.name());
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

    let phys_mem_offset = VirtAddr::new(boot_info.start_address() as u64);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let memory_map: MemoryMap = MemoryMap::from_memory_map_tag(boot_info.memory_map_tag().unwrap());
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    
    if let Some(framebuffer_tag_result) = boot_info.framebuffer_tag() {
        if let Ok(framebuffer_tag) = framebuffer_tag_result {
            framebuffer::map_framebuffer_page_table(&mut mapper, &mut frame_allocator, &framebuffer_tag);
        } else {
            panic!("Failed to get framebuffer tag");
        }
    } else {
        panic!("No framebuffer tag found");
    }

    if let Some(Ok(framebuffer_tag)) = boot_info.framebuffer_tag() {
        let addr = framebuffer_tag.address() as *mut u32;
        let width = framebuffer_tag.width();
        let height = framebuffer_tag.height();
        let bpp = framebuffer_tag.bpp();

        println!(
            "Framebuffer from address: {:#x} to address: {:#x}",
            addr as u64,
            (addr as u64) + (width * height * (bpp as u32) / 8) as u64
        );

        // check if address is identity mapped
        if is_identity_mapped(VirtAddr::new(addr as u64), &mapper) {
            println!("Framebuffer address is identity mapped");
        } else {
            println!("Framebuffer address is not identity mapped");
        }

        if bpp == 32 {
            let pitch = framebuffer_tag.pitch();
            make_screen_green(addr, width, height, pitch, bpp);

            // unsafe {
            //     println!("Pixel value before: {:#x}", *addr);
            //     *addr = 0x00FF00; // Set to green
            //     println!("Pixel value after: {:#x}", *addr);
            // }
        } else {
            println!("Unsupported bits per pixel: {}", bpp);
        }

        println!(
            "Framebuffer at: {:#x}, {}x{} ({} bpp)",
            addr as u64, width, height, bpp
        );
    } else {
        println!("No framebuffer tag found");
    }

    // unsafe {
    //     core::arch::asm!(
    //         "mov eax, 0x00FF00",     // Green color
    //         "mov [0xFD000000], eax", // Write to framebuffer address
    //     );
    // }

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

fn make_screen_green(framebuffer: *mut u32, width: u32, height: u32, pitch: u32, bpp: u8) {
    let green_color: u32 = 0x00FF0000; // Green in 32-bit ARGB

    unsafe {
        for y in 0..height {
            for x in 0..width {
                println!("x: {}, y: {}", x, y);
                let pixel_offset: u32 = (y * pitch + x * ((bpp as u32) / 8)).into();
                println!("pixel_offset: {}", pixel_offset);
                let pixel_ptr = framebuffer.add(pixel_offset as usize);
                println!("pixel_ptr: {:?}", pixel_ptr);
                println!("pixel_ptr value: {:?}", *pixel_ptr);

                *pixel_ptr = green_color;
                println!("After pixel_ptr value: {:?}", *pixel_ptr);
            }
        }
    }
}

fn is_identity_mapped(virtual_address: VirtAddr, mapper: &impl Translate) -> bool {
    if let Some(physical_address) = mapper.translate_addr(virtual_address) {
        // Compare physical and virtual addresses
        physical_address.as_u64() == virtual_address.as_u64()
    } else {
        // Address is not mapped at all
        false
    }
}
