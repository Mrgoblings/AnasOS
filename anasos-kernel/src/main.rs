#![no_std]
#![no_main]

use core::panic::PanicInfo;

use anasos_kernel::{ 
    println, 
    init, 
    hlt,
    memory,
};
use x86_64::{ structures::paging::Page, VirtAddr };

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);


pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    
    println!("Hello World{}", "!");

    init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    println!("Still Alive!");

    hlt();
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panicked: \n{}", info);
    anasos_kernel::hlt();
}
