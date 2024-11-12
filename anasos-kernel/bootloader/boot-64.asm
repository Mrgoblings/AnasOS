GLOBAL long_mode_start 
EXTERN _start

SECTION .text
BITS 64
long_mode_start:
    ; load null to all data segmant registers (needed for the cpu to work as intended)
    MOV ax, 0
    MOV ss, ax
    MOV ds, ax
    MOV es, ax
    MOV fs, ax
    MOV gs, ax

    CALL _start

    HLT

