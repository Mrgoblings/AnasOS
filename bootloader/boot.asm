BITS 64
section .text
    ALIGN 4
    DD 0x1BADB002
    DD 0x00000000
    DD -(0x1BADB002 + 0x00000000) ; checksum - to verify the bootloader

global start
extern _start ; rust start function

start:
    CLI
    MOV esp, stack_space
    CALL _start ; call the rust start function
    HLT
HaltKernel:
    CLI
    HLT
    JMP HaltKernel

section .bss
RESB 8192 ; bytes reserved for stack
stack_space:
