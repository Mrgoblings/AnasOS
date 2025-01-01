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
    DD -(0xE85250D6 + 0 + (header_end - header_start))

	; Framebuffer Request
;	DW 5                     ; Tag type
;	DW 0                     ; Flags: Optional
;	DD 20                    ; Size
;	DD 1024                  ; Width (example)
;	DD 768                   ; Height (example)
;	DD 32                    ; Bits per pixel (example)

    ; End Tag
    DW 0                     ; End tag type
    DW 0                     ; End tag flags
    DD 8                     ; End tag size
header_end: