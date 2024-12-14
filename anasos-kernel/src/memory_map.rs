// From https://github.com/rust-osdev/bootloader/blob/086c248e89ec3519f77dadda6c74d193e6fa22fd/bios/stage-2/src/memory_map.rs
// From http://wiki.osdev.org/Detecting_Memory_(x86)#Getting_an_E820_Memory_Map

use bootloader_x86_64_bios_common::E820MemoryRegion;

use core::arch::asm;

use lazy_static::lazy_static;
use spin::Mutex;


lazy_static! {
    pub static ref TOTAL_RAM: Mutex<u64> = Mutex::new(init_total_ram());
}

// Move the initialization of TOTAL_RAM into a non-constant context
fn init_total_ram() -> u64 {
    let memory_map = query_memory_map().unwrap();
    calculate_total_ram(&memory_map)
}


/// Calculate the total RAM in the system by summing up all usable memory regions.
fn calculate_total_ram(memory_map: &[E820MemoryRegion]) -> u64 {
    memory_map.iter()
        .filter(|region| region.region_type == 1) // Assuming 1 indicates usable RAM
        .map(|region| region.len)
        .sum()
}


/// Use the INT 0x15, eax= 0xE820 BIOS function to get a memory map
fn query_memory_map() -> Result<&'static mut [E820MemoryRegion], ()> {
    const SMAP: u32 = 0x534D4150;

    static mut MEMORY_MAP_BUFFER: [E820MemoryRegion; 128] = [E820MemoryRegion {
        start_addr: 0,
        len: 0,
        region_type: 0,
        acpi_extended_attributes: 0,
    }; 128];

    let memory_map = unsafe { &mut MEMORY_MAP_BUFFER };

    let mut i = 0;
    let mut offset = 0;
    let buf = [0u8; 24];
    loop {
        let ret: u32;
        let buf_written_len;
        unsafe {
            asm!(
                "push rbx",
                "mov rbx, rdx",
                "mov edx, 0x534D4150",
                "int 0x15",
                "mov rdx, rbx",
                "pop rbx",
                inout("eax") 0xe820 => ret,
                inout("rdx") offset,
                inout("ecx") buf.len() => buf_written_len,
                in("di") &buf
            )
        };
        if ret != SMAP {
            return Err(());
        }

        if buf_written_len != 0 {
            let buf = &buf[..buf_written_len];

            let (&base_raw, rest) = split_array_ref(buf);
            let (&len_raw, rest) = split_array_ref(rest);
            let (&kind_raw, rest) = split_array_ref(rest);
            let acpi_extended_raw: [u8; 4] = rest.try_into().unwrap_or_default();

            let len = u64::from_ne_bytes(len_raw);
            if len != 0 {
                memory_map[i] = E820MemoryRegion {
                    start_addr: u64::from_ne_bytes(base_raw),
                    len,
                    region_type: u32::from_ne_bytes(kind_raw),
                    acpi_extended_attributes: u32::from_ne_bytes(acpi_extended_raw),
                };
                i += 1;
            }
        }


        if offset == 0 {
            break;
        }
    }

    Ok(&mut memory_map[..i])
}

fn split_array_ref<const N: usize, T>(slice: &[T]) -> (&[T; N], &[T]) {
    if N > slice.len() {
        panic!("fail: split_array_ref index out of range.");
    }
    let (a, b) = slice.split_at(N);
    // SAFETY: a points to [T; N]? Yes it's [T] of length N (checked by split_at)
    unsafe { (&*(a.as_ptr() as *const [T; N]), b) }
}
