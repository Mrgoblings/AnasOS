SECTION .multiboot_header
ALIGN 8
header_start:
    ; Multiboot2 magic number
    DD 0xE85250D6            ; Magic number
    ; Architecture (protected mode i386)
    DD 0
    ; Header length
    DD header_end - header_start
    ; Checksum
    DD -(0xE85250D6 + 0 + (header_end - header_start))

    ; Console Flags Tag
    ALIGN 8
    DW 4                     ; Tag type: Console flags
    DW 0                     ; Flags: Optional
    DD 12                    ; Size of this tag
    DD 1                     ; Console flags (Bit 0 set)

    ; Framebuffer Tag
    ALIGN 8
    DW 5                     ; Tag type: Framebuffer
    DW 0                     ; Flags: Optional
    DD 20                    ; Size of this tag
    DD 1280                  ; Preferred framebuffer width
    DD 720                   ; Preferred framebuffer height
    DD 32                    ; Preferred bits per pixel

    ; End Tag
    ALIGN 8
    DD 0                     ; Tag type: End tag
    DD 8                     ; Size of end tag
header_end:
