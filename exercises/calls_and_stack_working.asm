[org 0x7c00]

mov bp, 0x8000
mov sp, bp
mov bh, 'A'
push bx
mov bh, 'B'

mov ah, 0x0e
mov al, bh
int 0x10

pop bx

mov ah, 0xe
mov al, bh,
int 0x10


call func
call func

mov ah, 0x0e
mov al, 'A'
int 0x10


jmp $

func:
    mov ah, 0x0e
    mov al, 'H'
    int 0x10
    ret


times 510 - ($ - $$) db 0
dw 0xaa55

