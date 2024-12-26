ENTRY(start_protected_mode)

SECTIONS
{
	. = 1M;

	.boot :
	{
		KEEP(*(.multiboot_header))
        *(.boot)
	}

	.text :
	{
		*(.text)
	}
}
