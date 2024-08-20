[org 0x7c00]
mov ax, 0
mov ss, ax
mov ds, ax
mov es, ax

call loopHomeStr
call readString

jmp $

homeStr: db "Enter a string :", 0x0D, 0x0A, 0

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


readString:
    push 0xFFFF ; push dummy element for end of the stack

loopRead:
    ; clear ah
    xor ah, ah
    
    int 0x16
    cmp al, 0x0D ; check if its enter ('\r')
    je printStack

    push ax
    jmp loopRead

printStack:
    pop ax
    
    cmp ax, 0xFFFF
    je end

    mov al, 0x0e
    int 0x10
    
    jmp printStack

end:
    ret

times 510 - ($ - $$) db 0
dw 0xaa55
