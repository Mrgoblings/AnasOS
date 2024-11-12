global long_mode_start 
extern _start

section .text
BITS 64
long_mode_start:
    ; load null to all data segmant registers (needed for the cpu to work corectly)
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; print "OK"
    MOV dword [0xb8000], 0x2f4b2f4f

    ; 

    HLT

