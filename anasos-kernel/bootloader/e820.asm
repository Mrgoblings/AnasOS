; From http://wiki.osdev.org/Detecting_Memory_(x86)#Getting_an_E820_Memory_Map

SECTION .boot
BITS 16

; Use the INT 0x15, eax= 0xE820 BIOS function to get a memory map
; Inputs: es:di -> destination buffer for 24-byte entries
; Outputs: bp = entry count, trashes all registers except esi

do_e820:
    xor ebx, ebx            ; EBX must be 0 to start
    xor bp, bp              ; Keep an entry count in BP
    mov edx, 0x534D4150     ; Place "SMAP" into EDX
    mov eax, 0xE820
    mov byte [es:di + 20], 1 ; Force a valid ACPI 3.X entry
    mov ecx, 24             ; Ask for 24 bytes
    int 0x15
    jc .failed              ; Carry set on first call means "unsupported function"
    mov edx, 0x534D4150     ; Repair potentially trashed EDX
    cmp eax, edx            ; On success, EAX must be reset to "SMAP"
    jne .failed
    test ebx, ebx           ; EBX = 0 implies list is only 1 entry long (worthless)
    je .failed
    jmp .jmpin

.e820lp:
    mov eax, 0xE820         ; EAX and ECX get trashed on every INT 0x15 call
    mov byte [es:di + 20], 1 ; Force a valid ACPI 3.X entry
    mov ecx, 24             ; Ask for 24 bytes again
    int 0x15
    jc .e820f               ; Carry set means "end of list already reached"
    mov edx, 0x534D4150     ; Repair potentially trashed EDX

.jmpin:
    jcxz .skipent           ; Skip any 0-length entries
    cmp cl, 20              ; Got a 24-byte ACPI 3.X response?
    jbe .notext
    test byte [es:di + 20], 1 ; If so: is the "ignore this data" bit clear?
    je .skipent

.notext:
    mov ecx, [es:di + 8]    ; Get lower uint32_t of memory region length
    or ecx, [es:di + 12]    ; "OR" it with upper uint32_t to test for zero
    jz .skipent             ; If length uint64_t is 0, skip entry
    inc bp                  ; Got a good entry: ++count, move to next storage spot
    add di, 24

.skipent:
    test ebx, ebx           ; If EBX resets to 0, list is complete
    jne .e820lp

.e820f:
    mov [mmap_ent], bp      ; Store the entry count
    clc                     ; Clear carry flag
    ret

.failed:
    stc                     ; Set carry flag to indicate error
    ret

GLOBAL mmap_ent
mmap_ent: dw 0
