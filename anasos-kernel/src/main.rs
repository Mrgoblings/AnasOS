#![no_std]
#![no_main]

use core::panic::PanicInfo;
use anasos_kernel::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    anasos_kernel::init(); 

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    println!("Still Alive!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
