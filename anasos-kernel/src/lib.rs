#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_kernel_main"]

extern crate alloc;
pub mod allocator;

pub mod interrupts;
pub mod vga;
pub mod gdt;
pub mod memory;

extern crate multiboot2;
use multiboot2::BootInformation;


pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start(_boot_info: &BootInformation) -> ! {
    init();
    test_kernel_main();
    hlt();
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
