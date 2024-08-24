[org 0x7c00]
; Tiny Memory Model
mov ax, 0
mov ss, ax 
mov es, ax
mov ds, ax


call printKur
call takeKur

jmp $

kur: db "Enter a number:", 0x0D, 0x0A, 0

saveKur: times 100 db 0

printKur:
    mov si, kur
printLoopKur:
    mov al, [si]
    cmp al, 0
    je endKur

    mov ah, 0x0e
    ; mov al, [si] napraven po gore
    int 0x10

    inc si

    jmp printLoopKur
endKur:
    ret

takeKur:
    mov si, saveKur

takeKurLoop:
    mov ah, 0
    int 0x16
    ; al - > butona koito sme cuknali

    cmp al, 0x0D
    je endTakeKur
    
    mov [si], al
    inc si

    mov ah, 0x0e
    int 0x10

    jmp takeKurLoop

endTakeKur:
    mov al, [si - 1]
    inc al
    mov [si - 1], al

    mov si, saveKur
    call printLoopKur
    
    mov ah, 0x0e
    
    mov al, 0x0D ; CR
    int 0x10
    
    mov al, 0x0A ; LF
    int 0x10

    ret
    
times 510 - ($ - $$) db 0
dw 0xaa55

