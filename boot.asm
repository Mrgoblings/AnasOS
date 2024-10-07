[org 0x7c00]

; disable all interupts
cli

; load the gdt descriptor
lgdt [GDT_Descriptor]

; change the last bit of cr0 (controll register 0) to 1
mov eax, cr0
or eax, 1
mov cr0, eax; yay 32 bit protected mode !!

; far jump - jump to another segment
jmp CODE_SEG: start_protected_mode


jmp $
GDT_Start:
    null_descriptptor: ; always a null descriptor at the begining of the descriptor
        dd 0 ; 00000000 - 8 bits
        dd 0;

    ; base (32 bits): 0
    ; limit (20 bits): 0xFFFFF
    ; present(1 bit), privilege (2 bits), type (1 bit - is Code/Data segment): 1001
    ; Type Flags(4 bits) (isCode, isConforming/isDirectionDown, read/write, 0 for the cpu) - 1010
    ; Other Flags (4 bits) ( isGranular - more free space, 32bit - for now yes, 64bit, AVL - some programs may need it so 0)- 1100
    code_descriptor:

        ; define the first 16 bits of the limit. The whole limit will be 0xffffF
        dw 0xFFFF 
        
        ; define the first 24 bits of the base
        dw 0 ; 16
        db 0 ; 8

        ; ppt + Type Flags (ppt -  pres, priv, type)
        db 0b10011010
        
        ; Other Flags + Last 4 bits of the limit(size)
        db 0b11001111
        
        ; Last 8 bits of base
        db 0

    ; base (32 bits): 0
    ; limit (20 bits): 0xFFFFF
    ; present(1 bit), privilege (2 bits), type (1 bit - is Code/Data segment): 1001
    ; Type Flags(4 bits) (isCode, isConforming/isDirectionDown, read/write, 0 for the cpu) - 0010
    ; Other Flags (4 bits) ( isGranular - more free space, 32bit - for now yes, 64bit, AVL - some programs may need it so 0)- 1100
    data_descriptor:

        ; define the first 16 bits of the limit. The whole limit will be 0xffffF
        dw 0xFFFF 
        
        ; define the first 24 bits of the base
        dw 0 ; 16
        db 0 ; 8

        ; ppt + Type Flags (ppt -  pres, priv, type)
        db 10010010b
        
        ; Other Flags + Last 4 bits of the limit(size)
        db 11001111b
        
        ; Last 8 bits of base
        db 0
    GDT_End:

GDT_Descriptor: 
    dw GDT_End - GDT_Start - 1  ; size
    dd GDT_Start                ; start

CODE_SEG equ code_descriptor - GDT_Start
DATA_SEG equ data_descriptor - GDT_Start
    ; equ - define constants


times 510 - ($ - $$) db 0
dw 0xAA55

[bits 32]
start_protected_mode:
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; priting characters to Video Memory directly
    ; videoMemory starts at 0xb8000
    ; first byte: character
    ; second byte: colour
    
    mov edi, 0xB8000
    mov al, 'A'
    mov ah, 0x0f ; the color black

    jmp $ 

