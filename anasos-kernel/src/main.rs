#![no_std]
#![no_main]

use core::panic::PanicInfo;
use anasos_kernel::{ println, init, hlt };

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    init(); 

    println!("Still Alive!");

    hlt();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    anasos_kernel::hlt();
}
