SECTION .multiboot_header
header_start:
	; magic number
	DD 0xE85250D6 ; multiboot2
	; architecture
	DD 0 ; protected mode i386
	; header length
	DD header_end - header_start
	; checksum
	DD 0x100000000 - (0xE85250D6 + 0 + (header_end - header_start))

	; end tag
	DW 0
	DW 0
	DD 8
header_end: