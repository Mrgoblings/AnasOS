ENTRY(start_protected_mode)

SECTIONS
{
    . = 1M; /* Set the code starting address to 1MB */
    .boot ALIGN(4096):
    {
        KEEP(*(.multiboot_header));
        *(.boot)
    }

  /* Kernel metadata section */
    .kernel ALIGN(4096):
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
        *(COMMON)         /* Uninitialized global variables */
        __bss_end = .;
    }

    /* Only Reference, Do Not Specify Sizes */
    _stack_start = stack_bottom;
    _stack_end = stack_top;
    _heap_start = heap_bottom;
    _heap_end = heap_top;

    .debug :
    {
        *(.debug_info)
        *(.debug_abbrev)
        *(.debug_line)
        *(.debug_str)
    }
   
}
