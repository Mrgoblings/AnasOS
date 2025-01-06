GLOBAL start_protected_mode

GLOBAL heap_bottom
GLOBAL heap_top

GLOBAL PML4
GLOBAL PDPT
GLOBAL PD
GLOBAL PT
GLOBAL stack_bottom
GLOBAL stack_top

EXTERN start_long_mode
EXTERN save_boot_info

SECTION .text
BITS 32

start_protected_mode:
    MOV esp, stack_top
    ; Save all general-purpose registers like EAX, EBX, ECX, etc.
    ; The registers are populated from the multiboot2 header and should be preserved
    ;   EAX: Multiboot2 magic number
    ;   EBX: Multiboot2 info pointer
    PUSHA           

    ; CALL print_ascii_art

    CALL check_multiboot
    CALL check_cpuid
    CALL check_long_mode

    CALL setup_page_tables
    CALL enable_paging

    POPA        ; Restore all general-purpose registers like EAX, EBX, ECX, etc.
    
    LGDT [gdt64.pointer]
    JMP gdt64.code_segment:start_long_mode

    HLT


check_multiboot:
    CMP eax, 0x36d76289
    JNE .no_multiboot
    RET
.no_multiboot:
    MOV al, "M"
    JMP error

check_cpuid:
    PUSHFD
    POP eax
    MOV ecx, eax
    XOR eax, 1 << 21
    PUSH eax
    POPFD
    PUSHFD
    POP eax
    PUSH ecx
    POPFD
    CMP eax, ecx
    JE .no_cpuid
    RET
.no_cpuid:
    MOV al, "C"
    JMP error

check_long_mode:
    MOV eax, 0x80000000
    CPUID
    CMP eax, 0x80000001
    JB .no_long_mode

    MOV eax, 0x80000001
    CPUID
    TEST edx, 1 << 29
    JZ .no_long_mode

    RET
.no_long_mode:
    MOV al, "L"
    JMP error

setup_page_tables:
    ; Identity mapping: map each virtual address to the same physical address

    ; Initialize the level 4 page table (PML4)
    MOV eax, PDPT
    OR eax, 0b11 ; Present, Writable
    MOV [PML4], eax

    ; Initialize the level 3 page table (PDPT)
    MOV eax, PD
    OR eax, 0b11 ; Present, Writable
    MOV [PDPT], eax

    ; Initialize the level 2 page table (PD) with three PDEs
    MOV eax, PT                ; First page table
    OR eax, 0b11               ; Present, Writable
    MOV [PD], eax

    MOV eax, PT + 0x1000       ; Second page table
    OR eax, 0b11               ; Present, Writable
    MOV [PD + 8], eax          ; Write second PDE

    MOV eax, PT + 0x2000       ; Third page table
    OR eax, 0b11               ; Present, Writable
    MOV [PD + 16], eax         ; Write third PDE

    MOV ecx, 3                ; Number of page tables to fill (3 iterations)
    XOR edi, edi              ; Initialize loop counter (edi = 0)

.fill_loop:
    MOV eax, PT               ; Base of the first page table
    ADD eax, edi              ; Adjust base by loop counter (edi * 0x1000)
    SHL edi, 1                ; Calculate offset in MiB (edi * 2)
    MOV ebx, edi              ; Set the offset (2 MiB chunks)
    SHR edi, 1                ; Restore edi (undo the shift for next iteration)
    CALL fill_page_table      ; Call the function to fill the page table
    ADD edi, 0x1000           ; Move to the next page table (PT + 0x1000 * i)
    LOOP .fill_loop            ; Decrement ecx and loop if ecx > 0

    RET


enable_paging:
    ; Pass the page table location to the CPU
    MOV eax, PML4           ; Load physical address of PML4
    MOV cr3, eax

    ; Enable Physical Address Extension (PAE)
    MOV eax, cr4
    OR eax, 1 << 5          ; Set PAE bit
    MOV cr4, eax

    ; Enable Long Mode
    MOV ecx, 0xC0000080     ; MSR for EFER
    RDMSR
    OR eax, 1 << 8          ; Set LME (Long Mode Enable)
    WRMSR

    ; RET
    
    ; Enable Paging
    MOV eax, cr0
    OR eax, 1 << 31         ; Set PG bit
    MOV cr0, eax

    MOV eax, eax

    RET 


; Function to handle errors
; Input:
; - AL: Error code
; Output:
; - Prints an error message to the screen
; "ERR: X", where X is the error code from AL
error:
    MOV dword [0xB8000], 0x4F524F45
    MOV dword [0xB8004], 0x4F3A4F52
    MOV dword [0xB8008], 0x4F204F20
    MOV byte  [0xB800C], al
    HLT

; Function to fill a page table
; Input:
; - EAX: Base address of the page table
; - EBX: Starting offset in 2 MiB chunks (physical address / 2 MiB)
fill_page_table:
    PUSH ecx              ; Save ECX (used as the loop counter)
    MOV ecx, 0            ; Reset loop counter

.loop_fill_pt:
    MOV edx, ecx          ; Current page index
    ADD edx, ebx           ; Add the starting offset from EBX
    SHL edx, 12           ; Convert to physical address (4 KiB pages)
    OR edx, 0b11          ; Mark Present and Writable
    MOV [eax + ecx * 8], edx ; Write the entry to the page table

    INC ecx
    CMP ecx, 512          ; Fill all 512 entries (2 MiB range)
    JL .loop_fill_pt

    POP ecx               ; Restore ECX
    RET


write_W_to_vga:
    ; Write the letter "W" to the VGA text buffer
    mov edi, 0xB8000      ; VGA text buffer address
    mov ax, 0x0F57        ; "W" (ASCII 0x57) with attribute 0x0F (white on black)
    mov word [edi], ax    ; Write the word (character + attribute) to VGA memory
    HLT
    ; ret                   ; Return to the caller


SECTION .data
ALIGN 4096
ascii_art:
    db "     __    _   _    __    ____    ____     _____", 0x0F, 10, 0
    db "    /  \\  | \\ | |  /  \\  / ___7  / __ \\   / ____>", 0x0F, 10, 0
    db "   / /\\ \\ |  \\| | / /\\ \\ \\___ \\ | |  | | | (_____", 0x0F, 10, 0
    db "  / ____ \\| |\\  |/ ____ \\___) | | |  | |  \\____  \\", 0x0F, 10, 0
    db " / /    \\ \\_| \\_|_/    \\_\\____/ | |  | |  _____) |", 0x0F, 10, 0
    db "/_|      \\_\\                     \\____/  |______/", 0x0F, 10, 0
    db 0  ; Null terminator for end of data


SECTION .bss
ALIGN 4096
start_page_table:
PML4:
    RESB 4096                ; Level 4 Page Table 512 entries by 8 bytes each
PDPT:
    RESB 4096                ; Level 3 Page Table
PD:
    RESB 4096                ; Level 2 Page Table
PT:
    RESB 4096 * 3            ; Three Level 1 Page Tables (512 entries each)
end_page_table:
stack_bottom:
    RESB 4096 * 5 ; bytes reserved for stack (5 pages)
stack_top:
heap_bottom:
    RESB 100 * 1024 ; 100 KiB reserved for heap
heap_top:

SECTION .rodata
gdt64:
    dq 0 ; zero entry
.code_segment: EQU $ - gdt64
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53) ; 64-bit code segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64
