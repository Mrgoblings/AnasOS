#![no_std]
#![no_main]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use lazy_static::lazy_static;
use core::panic::PanicInfo;

use anasos_kernel::{ 
    allocator, bootinfo::{self, traits::BootInfo}, hlt, init, memory::{ self, BootInfoFrameAllocator}, println
};
use x86_64::VirtAddr;

// Symbols defined in `linker.ld`


#[no_mangle]
pub extern "C" fn _start() -> ! {
    lazy_static! {
        static ref BOOT_INFO: BootInfo = unsafe { bootinfo::get() };
    }

    println!("boot_info: {:?}", *BOOT_INFO);

    // #[cfg(notest)]
    kernel_main(&*BOOT_INFO);

    // #[cfg(test)]
    // test_kernel_main(&*BOOT_INFO);

    hlt();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panicked: \n{}", info);
    hlt();
}


fn kernel_main(boot_info: &'static BootInfo) {
    println!("Hello World{}", "!");
    init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );
    

    println!("Still Alive!");
}

fn test_kernel_main(_boot_info: &'static BootInfo) {
    println!("Running tests");
    // test code here
    println!("Tests passed");
}