#![no_std]
#![no_main]

extern crate alloc;
use core::panic::PanicInfo;

use anasos_kernel::{ 
    allocator, hlt, init, memory::{ self, memory_map::{FromMemoryMapTag, MemoryMap}, BootInfoFrameAllocator}, 
    println, task::{executor::Executor, keyboard, Task}
};
use x86_64::VirtAddr;


extern crate multiboot2;
use multiboot2::{BootInformation, BootInformationHeader};


#[no_mangle]
pub extern "C" fn _start(mb_magic: u32, mbi_ptr: u32) -> ! {    
    if mb_magic != multiboot2::MAGIC {
        panic!("Invalid Multiboot2 magic number");
    }

    let boot_info = unsafe { BootInformation::load(mbi_ptr as *const BootInformationHeader).unwrap() };
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

    if let Some(Ok(framebuffer_tag)) = boot_info.framebuffer_tag() {
        let addr = framebuffer_tag.address();
        let width = framebuffer_tag.width();
        let height = framebuffer_tag.height();
        let bpp = framebuffer_tag.bpp();

        println!("Framebuffer at: {:#x}, {}x{} ({} bpp)", addr, width, height, bpp);
    } else {
        println!("No framebuffer tag found");
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
    hlt();
}


fn kernel_main(boot_info: &BootInformation) -> ! {
    println!("Hello World{}", "!");
    init();

    let phys_mem_offset = VirtAddr::new(boot_info.start_address() as u64);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let memory_map: MemoryMap = MemoryMap::from_memory_map_tag(boot_info.memory_map_tag().unwrap());
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");


    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run(); // This function will never return
}

fn test_kernel_main(_boot_info: & BootInformation) -> ! {
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