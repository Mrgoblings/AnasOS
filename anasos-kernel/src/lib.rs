#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// extern crate alloc;
// pub mod allocator;

pub mod interrupts;
pub mod vga;
pub mod gdt;
// pub mod memory_map;
// pub mod memory;

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
// use bootloader::{entry_point, BootInfo};

#[cfg(test)]
// entry_point!(test_kernel_main);

#[cfg(test)]
#[no_mangle]
// pub fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt();
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
