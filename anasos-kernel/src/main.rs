#![no_std]
#![no_main]

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("AAA Hello!!!!@@");
    loop {}
}

pub fn print(input: &str) {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, byte) in input.bytes().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

/*
pub fn print(input: &[u8]){
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in input.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}
*/