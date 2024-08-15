[org 0x7c00]
mov ah, 0x0e
mov bx, testString


printString:
    mov al, [bx]
    cmp al, 0
    je endPrintString

    int 0x10
    inc bx
    jmp printString
endPrintString:


getChar:
    mov ah, 0
    int 0x16
    
    mov ah, 0x0e
    int 0x10

newLine:
    mov al, 0x0d
    int 0x10
    mov al, 0x0a
    int 0x10

mov cx, 0
testGetBufferAndPrint:
    cmp cx, 10
    je endTestGetBufferAndPrint

    ; get key
    mov ah, 0
    int 0x16

    ; print
    mov ah, 0x0e
    int 0x10
    
    ; save
    mov [bx], al

    inc bx
    inc cx
    jmp testGetBufferAndPrint
endTestGetBufferAndPrint:
    mov bx, 0

newLine2:
    mov al, 0x0d
    int 0x10
    mov al, 0x0a
    int 0x10

mov bx, buffer
printBuffer:
    mov ah, 0x0e
    mov al, [bx]
    
    cmp al, 0
    je endPrintBuffer 

    int 0x10

    inc bx
    jmp printBuffer

endPrintBuffer:

jmp $

; Data Section
testString:
    db "Enter a Letter:", 0x0d, 0x0a, 0

buffer:
    times 10 db 0

times 510 - ($ - $$) db 0
dw 0xaa55

