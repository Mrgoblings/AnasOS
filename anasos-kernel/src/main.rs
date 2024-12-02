#![no_std]
#![no_main]

use core::panic::PanicInfo;
use anasos_kernel::{ println, init, hlt };

// make the module accessible to the bootloader

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

// mod paging;

pub fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    println!("Hello World{}", "!");

    init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());


    println!("Still Alive!");

    hlt();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    anasos_kernel::hlt();
}
