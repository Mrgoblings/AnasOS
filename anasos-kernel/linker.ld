ENTRY(start)

SECTIONS
{
    /* Start address as per GRUB multiboot standard */
    . = 1M;

    /* Boot section: includes the multiboot header */
    .boot :
    {
        KEEP(*(.multiboot_header))
    }

    /* Text section: contains code */
    .text :
    {
        *(.text)
    }

    /* Reserve a buffer for kernel loading */
    . = 0x500;
    _kernel_buffer = .;
    . += 512;

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
        KEEP(*(.kernel))
    }
}
