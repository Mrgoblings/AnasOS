#![no_std]
#![no_main]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use core::panic::PanicInfo;

use anasos_kernel::{ 
    allocator, hlt, init, memory::{ self, memory_map::{FromMemoryMapTag, MemoryMap}, BootInfoFrameAllocator}, print, println
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

    hlt();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panicked: \n{}", info);
    hlt();
}


fn kernel_main(boot_info: &BootInformation) {
    println!("Hello World{}", "!");
    init();

    let phys_mem_offset = VirtAddr::new(boot_info.start_address() as u64);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let memory_map: MemoryMap = MemoryMap::from_memory_map_tag(boot_info.memory_map_tag().unwrap());
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());
    println!("vec[499] = {}\n", vec[499]);

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );
    

    println!("Still Alive!");
}

fn test_kernel_main(_boot_info: & BootInformation) {
    println!("Running tests");
    // test code here
    println!("Tests passed");
}