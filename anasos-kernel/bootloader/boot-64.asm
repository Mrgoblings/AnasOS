GLOBAL start_long_mode 
EXTERN _start

SECTION .text
BITS 64
start_long_mode:
    ; load null to all data segmant registers (needed for the cpu to work as intended)
    ; documented here - https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#Machine-state
    PUSH RAX
    XOR AX, AX
    MOV DS, AX
    MOV ES, AX
    MOV FS, AX
    MOV GS, AX
    MOV SS, AX

    POP RAX

    CALL _start
    
    ; ; Write the letter "W" to the VGA text buffer
    ; MOV rdi, 0xB8000       ; VGA text buffer address (identity-mapped in page tables)
    ; MOV ax, 0x0F53         ; "S" (ASCII 0x53) with attribute 0x0F (white on black)
    ; MOV word [rdi], ax     ; Write the word (character + attribute) to the VGA buffer


    HLT


