GLOBAL long_mode_start 
EXTERN _start

SECTION .text
BITS 64
long_mode_start:
    ; load null to all data segmant registers (needed for the cpu to work as intended)
    ; documented here - https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#Machine-state
    MOV ax, 0
    MOV ds, ax
    MOV es, ax
    MOV fs, ax
    MOV gs, ax
    MOV ss, ax

    CALL _start

    HLT

