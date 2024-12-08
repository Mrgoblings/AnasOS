#![no_std]
#![no_main]

use core::panic::PanicInfo;

use anasos_kernel::{ 
    println, 
    init, 
    hlt,
    memory::{ self, BootInfoFrameAllocator},
    allocator,
};
use x86_64::{ structures::paging::Page, VirtAddr };

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);


pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    
    println!("Hello World{}", "!");

    init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    
    let mut mapper = unsafe { 
        memory::init(phys_mem_offset) 
    };
    
    let mut frame_allocator = unsafe { 
        BootInfoFrameAllocator::init(&boot_info.memory_map) 
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let x = Box::new(41);

    println!("Still Alive!");

    hlt();
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panicked: \n{}", info);
    anasos_kernel::hlt();
}
