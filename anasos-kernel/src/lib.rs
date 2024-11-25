#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod vga;

pub fn init() {
    interrupts::init_idt();
}
