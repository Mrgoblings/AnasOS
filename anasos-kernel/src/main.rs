#![no_std]
#![no_main]

use core::panic::PanicInfo;
use anasos_kernel::{ 
    println, 
    init, 
    hlt,
    memory,
};
use x86_64::{ structures::paging::Translate, VirtAddr };

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);


pub fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    
    println!("Hello World{}", "!");

    init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset.into_option().unwrap(),
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    println!("Still Alive!");

    hlt();
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panicked: \n{}", info);
    anasos_kernel::hlt();
}
