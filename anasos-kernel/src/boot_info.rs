 
pub struct BootInfo {
    pub api_version: ApiVersion,
    pub memory_regions: MemoryRegions,
    pub framebuffer: Optional<FrameBuffer>,
    pub physical_memory_offset: Optional<u64>,
    pub recursive_index: Optional<u16>,
    pub rsdp_addr: Optional<u64>,
    pub tls_template: Optional<TlsTemplate>,
    pub ramdisk_addr: Optional<u64>,
    pub ramdisk_len: u64,
    pub kernel_addr: u64,
    pub kernel_len: u64,
    pub kernel_image_offset: u64,
    pub _test_sentinel: u64,
}



extern "C" fn save_boot_info() {
    let boot_info = BootInfo {
        physical_memory_offset: x86_64::PhysAddr::,
        memory_map: MemoryMap::new(),
    };

    let boot_info_ptr = &boot_info as *const BootInfo;

    unsafe {
        asm!("mov rdi, {}", in(reg) boot_info_ptr);
    }
} 