BITS 64
section .multiboot
ALIGN 8
MULTIBOOT_HEADER:
    DD 0x1BADB002
    DD 0x00000000
    DD -(0x1BADB002 + 0x00000000) ; checksum - to verify the bootloader

section .text
global start
extern _start ; rust start function

start:
    CLI
    MOV rsp, stack_space + 8192
    AND rsp, -16
    CALL _start ; call the rust start function
    HLT

HaltKernel:
    CLI
    HLT
    JMP HaltKernel

section .bss
    RESB 8192 ; bytes reserved for stack
stack_space:
