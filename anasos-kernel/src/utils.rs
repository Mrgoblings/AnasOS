#![no_std]
#![no_main]

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, val: u8, count: usize) -> *mut u8 {
    for i in 0..count {
        *dest.add(i) = val;
    }
    dest
}