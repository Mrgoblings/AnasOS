GLOBAL start
EXTERN long_mode_start

SECTION .text
BITS 32

start:
    MOV esp, stack_top
    CALL check_multiboot
    call check_cpuid
    CALL check_long_mode

    CALL setup_page_tables
    CALL enable_paging

    LGDT [gdt64.pointer]
    JMP gdt64.code_segment:long_mode_start

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
    ; Identity mapping: each virtual address maps to the same physical address

    ; Initialize the level 4 page table
    MOV eax, page_table_l3
    OR eax, 0b11 ; present, writable
    MOV [page_table_l4], eax

    ; Initialize the level 3 page table
    MOV eax, page_table_l2
    OR eax, 0b11 ; present, writable
    MOV [page_table_l3], eax

    ; Initialize the level 2 page table
    MOV eax, page_table_l1
    OR eax, 0b11 ; present, writable
    MOV [page_table_l2], eax

    ; Fill the level 1 page table with 4 KiB page mappings
    MOV ecx, 0 ; counter
.loop_setup_page_tables:
    MOV eax, 0x1000  ; 4 KiB
    MUL ecx          ; calculate physical address
    OR eax, 0b11     ; present, writable
    MOV [page_table_l1 + ecx * 8], eax

    INC ecx
    CMP ecx, 512 ; fill all entries in level 1 page table (512 * 4 KiB = 2 MiB)
    JNE .loop_setup_page_tables

    RET

enable_paging:
    ; pass the page table location to the cpu
    MOV eax, page_table_l4
    MOV cr3, eax

    ; enable Phisical Address Extension (PAE)
    MOV eax, cr4
    OR eax, 1 << 5
    MOV cr4, eax

    ; enable long mode
    MOV ecx, 0xC0000080
    RDMSR ; Read Model Specific Register instruction
    OR eax, 1 << 8
    WRMSR ; Write Model Specific Register instruction

    ; enable paging
    MOV eax, cr0
    OR eax, 1 << 31
    MOV cr0, eax

    RET

error:
    ; print "ERR: X", where X is the error code
    MOV dword [0xB8000], 0x4F524F45
    MOV dword [0xB8004], 0x4F3A4F52
    MOV dword [0xB8008], 0x4F204F20
    MOV byte  [0xB800C], al
    HLT

SECTION .bss
ALIGN 4096
page_table_l4:
    RESB 4096
page_table_l3:
    RESB 4096
page_table_l2:
    RESB 4096
page_table_l1:
    RESB 4096 * 512 ; full page table
stack_bottom:
    RESB 4096 * 5 ; bytes reserved for stack (5 pages)
stack_top:

SECTION .rodata
gdt64:
    dq 0 ; zero entry
.code_segment: EQU $ - gdt64
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53) ; code segment 
.pointer:
    dw $ - gdt64 - 1
    dq gdt64
