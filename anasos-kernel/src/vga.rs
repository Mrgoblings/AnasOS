

pub fn print(input: &str) {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, byte) in input.bytes().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}