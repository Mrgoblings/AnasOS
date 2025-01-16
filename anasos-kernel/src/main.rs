#![no_std]
#![no_main]

extern crate alloc;
use core::panic::PanicInfo;

use anasos_kernel::{
    allocator, framebuffer, framebuffer_theseus::{self, color, pixel::{AlphaPixel, Pixel, RGBPixel}, Framebuffer}, hlt, init, memory::{
        self, create_example_mapping, memory_map::{FromMemoryMapTag, MemoryMap}, BootInfoFrameAllocator
    }, println, serial_println, task::{executor::Executor, keyboard, Task},
    framebuffer_off
};
use x86_64::{structures::paging::{frame, Translate}, PhysAddr, VirtAddr};


extern crate multiboot2;
use multiboot2::{BootInformation, BootInformationHeader};

#[no_mangle]
pub extern "C" fn _start(mb_magic: u32, mbi_ptr: u32) -> ! {
    if mb_magic != multiboot2::MAGIC {
        panic!("Invalid Multiboot2 magic number");
    }

    let boot_info =
        unsafe { BootInformation::load(mbi_ptr as *const BootInformationHeader).unwrap() };
    let _cmd = boot_info.command_line_tag();

    if let Some(bootloader_name) = boot_info.boot_loader_name_tag() {
        println!("Bootloader: {:?}", bootloader_name.name());
    }

    // Access the memory map
    if let Some(memory_map_tag) = boot_info.memory_map_tag() {
        for area in memory_map_tag.memory_areas() {
            println!(
                "Memory area: start = {:#x}, length = {:#x}, type = {:?}",
                area.start_address(),
                area.size(),
                area.typ()
            );
        }
    }

    println!("");

    // #[cfg(notest)]
    kernel_main(&boot_info);

    // #[cfg(test)]
    // test_kernel_main(&*BOOT_INFO);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panicked: \n{}", info);
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);

    hlt();
}

fn kernel_main(boot_info: &BootInformation) -> ! {
    println!("Kernel Start:");
    init();

    // println!("boot_info start");
    // println!("{:#?}", boot_info);
    // println!("boot_info end");



    // TODO: THIS is bulshit
    let phys_mem_offset = VirtAddr::new(boot_info.start_address() as u64);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut memory_map: MemoryMap = MemoryMap::from_memory_map_tag(boot_info.memory_map_tag().unwrap());
    
    let framebuffer_tag = boot_info.framebuffer_tag().unwrap().ok().ok_or("No framebuffer tag found").unwrap();
    let framebuffer_phys_addr = PhysAddr::new(framebuffer_tag.address());
    let framebuffer_start = framebuffer_tag.address() as u64;
    let framebuffer_size = framebuffer_tag.pitch() as u64 * framebuffer_tag.height() as u64;
    // let framebuffer_virt_addr = VirtAddr::new(0xFFFF8000_0000_0000); // Example virtual address
    let framebuffer_width = framebuffer_tag.width() as u64;
    let framebuffer_height = framebuffer_tag.height() as u64;

    println!("Framebuffer width: {}, height: {}", framebuffer_width, framebuffer_height);

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&mut memory_map, framebuffer_start, framebuffer_start + framebuffer_size) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");
    //framebuffer::init(&framebuffer_tag, &mut mapper, &mut frame_allocator);
    
    
    // framebuffer::map_framebuffer(
    //     framebuffer_phys_addr,
    //     framebuffer_size,
    //     framebuffer_virt_addr,
    //     &mut mapper,
    //     &mut frame_allocator,
    // ).expect("Failed to map framebuffer");

    // Initialize the framebuffer
    let mut framebuffer: Framebuffer<AlphaPixel> = Framebuffer::new(
        framebuffer_tag.width() as usize,
        framebuffer_tag.height() as usize,
        Some(framebuffer_phys_addr),
        &mut mapper,
        &mut frame_allocator,
    )
    .expect("Failed to initialize framebuffer");

    if is_identity_mapped(VirtAddr::new(framebuffer_phys_addr.as_u64()), &mapper) {
        println!("Framebuffer identity mapped to address: {:?}", framebuffer_phys_addr.as_u64());
    } else {
        println!("Framebuffer not identity mapped");
    }

    println!("Framebuffer identity mapped to address: {:?}", framebuffer.buffer_mut().as_ptr());
    
    println!(
        "Framebuffer info: width = {}, height = {}",
        framebuffer_width, framebuffer_height
    );

    // framebuffer::map_framebuffer_page_table(&mut mapper, &mut frame_allocator, framebuffer_tag);
    // framebuffer::check_framebuffer_mapping(&mapper, framebuffer_tag);

    // unsafe {
    //     println!("Pixel value before: {:#x}", *(framebuffer_virt_addr.as_mut_ptr::<u32>()));
    //     *(framebuffer_virt_addr.as_mut_ptr::<u32>()) = 0x00FF00; // Set to green
    //     println!("Pixel value after: {:#x}", *(framebuffer_virt_addr.as_mut_ptr::<u32>()));
    // }


    // Fill the screen with green
    let green_pixel: AlphaPixel = color::GREEN.into();
    framebuffer.overwrite_pixel(3, 3, green_pixel);
    // framebuffer.fill(green_pixel);

    // println!("Screen filled with green color.");


// unsafe {
//     core::arch::asm!(
//         "mov eax, 0x00FF00",     // Green color
//         "mov [0xFD000000], eax", // Write to framebuffer address
//     );
// }


    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run(); // This function will never return
}


fn test_kernel_main(_boot_info: &BootInformation) -> ! {
    println!("Running tests");
    // test code here
    println!("Tests passed");

    hlt();
}

// Test async functions
async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

// fn make_screen_green(framebuffer: *mut u32, width: u32, height: u32, pitch: u32, bpp: u8) {
//     let green_color: u32 = 0x00FF0000; // Green in 32-bit ARGB

//     unsafe {
//         for y in 0..height {
//             for x in 0..width {
//                 println!("x: {}, y: {}", x, y);
//                 let pixel_offset: u32 = (y * pitch + x * ((bpp as u32) / 8)).into();
//                 println!("pixel_offset: {}", pixel_offset);
//                 let pixel_ptr = framebuffer.add(pixel_offset as usize);
//                 println!("pixel_ptr: {:?}", pixel_ptr);
//                 println!("pixel_ptr value: {:?}", *pixel_ptr);

//                 *pixel_ptr = green_color;
//                 println!("After pixel_ptr value: {:?}", *pixel_ptr);
//             }
//         }
//     }
// }

fn is_identity_mapped(virtual_address: VirtAddr, mapper: &impl Translate) -> bool {
    if let Some(physical_address) = mapper.translate_addr(virtual_address) {
        // Compare physical and virtual addresses
        physical_address.as_u64() == virtual_address.as_u64()
    } else {
        // Address is not mapped at all
        false
    }
}
