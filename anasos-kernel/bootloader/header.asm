SECTION .multiboot_header
ALIGN 8
header_start:
    ; Multiboot2 magic number
    DD 0xE85250D6            ; Magic number
    ; Architecture (protected mode i386)
    DD 0
    ; Header length (updated dynamically)
    DD header_end - header_start
    ; Checksum
    DD 0x100000000 - (0xE85250D6 + 0 + (header_end - header_start))

    ; Memory Map Request
    DW 4                     ; Type
    DW 0                     ; Flags: Optional
    DD 16                    ; Size
    DD 0                     ; Reserved

    ; Framebuffer Request
    DW 5                     ; Type
    DW 0                     ; Flags: Optional
    DD 20                    ; Size
    DD 1024                  ; Width (example: 1024 pixels)
    DD 768                   ; Height (example: 768 pixels)
    DD 32                    ; Bits per pixel (example: 32 bpp)

    ; Bootloader Name Request
    DW 2                     ; Type
    DW 0                     ; Flags: Optional
    DD 16                    ; Size

    ; Command Line Request
    DW 1                     ; Type
    DW 0                     ; Flags: Optional
    DD 16                    ; Size

    ; Higher-half Kernel Request (Optional)
    ; If your kernel runs in the higher-half memory (like `0xFFFFFFFF80000000`),
    ; inform the bootloader to load it properly.
    DW 9                     ; Type
    DW 0                     ; Flags: Optional
    DD 16                    ; Size
    DD 0                     ; Reserved

    ; End Tag
    DW 0                     ; End tag type
    DW 0                     ; End tag flags
    DD 8                     ; End tag size
header_end: