#![no_std]
#![no_main]

use core::panic::PanicInfo;
use anasos_kernel::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    anasos_kernel::init(); 

    // // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    // example of a stack overflow reboot sequence didn't work - no reboot sequence
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }
    // // trigger a stack overflow
    // stack_overflow();

    println!("Still Alive!");

    anasos_kernel::hlt();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    anasos_kernel::hlt();
}
