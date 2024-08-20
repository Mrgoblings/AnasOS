[org 0x7c00]
xor ax, ax
mov ss, ax
mov ds, ax
; mov es, ax

call printHomeStr
call readString

jmp $


; section .data ; -- sections dont work in the tiny memory model
homeStr: db "Enter a string :", 0x0D, 0x0A, 0


; section .text ; -- not working in the tiny memory model
printHomeStr:
    mov si, homeStr

loopHomeStr:
    mov al, [si]
    
    cmp al, 0
    je endHome

    mov ah, 0x0e
    int 0x10

    inc si
    jmp loopHomeStr
endHome:
    ret


inputStr: times 100 db 0

readString:
    mov si, inputStr

loopRead:
    ; clear ah
    xor ah, ah
    
    ; get input
    int 0x16

    ; print input
    mov ah, 0x0e
    int 0x10

    mov [si], al
    inc si
    
    cmp al, 0x0D ; check if its enter ('\r')
    je printInput

    jmp loopRead


printInput:
    ; finish the new line CR <LN> part
    mov ah, 0x0e,
    mov al, 0x0A
    int 0x10

    mov si, inputStr

printLoop:
    mov al, [si]
    inc si
    cmp al, 0x0D ; the last element was '\r'
    je end

    mov ah, 0x0e
    int 0x10
    
    jmp printLoop

end:
    ret

times 510 - ($ - $$) db 0
dw 0xaa55
