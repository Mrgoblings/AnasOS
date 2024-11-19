GLOBAL long_mode_start
EXTERN _start

SECTION .text
BITS 64

long_mode_start:
    ; Clear segment registers (required in long mode)
    XOR ax, ax
    MOV ss, ax
    MOV ds, ax
    MOV es, ax
    MOV fs, ax
    MOV gs, ax

    ; Call the kernel entry point
    CALL _start

    ; Halt the CPU (should not return here)
    HLT
