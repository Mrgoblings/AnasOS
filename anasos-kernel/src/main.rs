#![no_std]
#![no_main]

use core::panic::PanicInfo;

// mod boot_info;

use anasos_kernel::{ 
    println, 
    init, 
    hlt,
    // TOTAL_RAM,
    // memory::{ self, BootInfoFrameAllocator},
    // allocator,
};
// use x86_64::{ structures::paging::Page, VirtAddr };

// use bootloader::{BootInfo, entry_point};

// entry_point!(kernel_main);

#[no_mangle]
pub extern "C" fn _start(/*boot_info: &'static BootInfo*/) -> ! {
    
    println!("Hello World{}", "!");


    init();

    // println!("Total RAM: {} bytes", TOTAL_RAM);
    

    println!("Still Alive!");

    hlt();
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panicked: \n{}", info);
    anasos_kernel::hlt();
}
