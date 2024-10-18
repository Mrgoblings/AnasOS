#![no_std]
#![no_main]

use core::panic::PanicInfo;
// use core::fmt::Write;

mod vga;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {}
}