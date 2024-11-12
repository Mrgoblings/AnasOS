global long_mode_start 
extern _start

section .text
BITS 64
long_mode_start:
    ; load null to all data segmant registers (needed for the cpu to work corectly)
    MOV ax, 0
    MOV ss, ax
    MOV ds, ax
    MOV es, ax
    MOV fs, ax
    MOV gs, ax

    ; print "OK"
    ; MOV dword [0xb8000], 0x2f4b2f4f

    CALL _start
    ; 

    HLT

