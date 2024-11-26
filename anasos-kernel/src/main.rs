#![no_std]
#![no_main]

use core::panic::PanicInfo;
use anasos_kernel::{println, print};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    anasos_kernel::init(); 

    println!("Still Alive!");

    anasos_kernel::hlt();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    anasos_kernel::hlt();
}
