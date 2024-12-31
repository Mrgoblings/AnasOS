ENTRY(start_protected_mode)

SECTIONS
{
    . = 0x0;
    .boot :
    {
        KEEP(*(.multiboot_header));
        *(.boot)
    }

    /* Page table addresses already defined in boot.asm, but ensure alignment */
    . = ALIGN(4096); /* Page size */
    __page_table_start = .;
	_p4 = PML4;         /* Reference PML4 address from boot.asm */
	_p3 = PDPT;         /* Reference PDPT address from boot.asm */
	_p2 = PD;           /* Reference PD address from boot.asm */
	_p1 = PT;           /* Reference PT address from boot.asm */
	__page_table_end = .;

    /* Stack setup handled in boot.asm */
    _stack_start = stack_bottom; /* Stack defined in boot.asm */
    _stack_end = stack_top;

    /* Memory map section for bootloader metadata */
    __bootloader_start = .;
    _memory_map = .;  /* Memory map entry */
    . += 4096;		/* Reserve 4KB for memory map */

    /* Kernel metadata section */
    .kernel :
    {
        _kernel_start_addr = .;
        *(.text)
        *(.data)
        _kernel_end_addr = .;
    }

    _kernel_size = _kernel_end_addr - _kernel_start_addr;
    

    .bss ALIGN(4096):
    {
        __bss_start = .;
        *(.bss)
        *(COMMON)
        __bss_end = .;
    }
}
