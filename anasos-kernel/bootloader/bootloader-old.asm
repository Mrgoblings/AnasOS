BITS 32
; ALIGN 4

global start
section .text
; extern _start ; rust start function

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
