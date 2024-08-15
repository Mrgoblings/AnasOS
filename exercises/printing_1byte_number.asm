; mov ds, 0x7c0
[org 0x7c00]

; Tiny Memory Model
xor ax, ax ; cool way to set ax to 0. XOR with itself is always 0
mov ss, ax
mov ds, ax

mov cl, 127
call printNumber
jmp $   


printNumber:
    ; check if the number in cl is 0
    cmp cl, 0
    je end

    ; setup for deviding
    mov bl, 10
    push 0xFFFF ; dummy symbol for knowing when is the end of the stack
loopCharsToStack:
    xor ah, ah

    ; bl % 10
    mov al, cl
    div bl
    
    ; save for nex iteration 
    mov cl, al

    ; save the remainder as ascii number to ssppl, ah
    mov al, ah
    add al, '0'
    push ax

    ; cycle
    test cl, cl ; cool way to check if cl is 0. makes an AND operation on itself and sets flags like cmp
    jne loopCharsToStack

printBuffer:
    pop ax

    ; end of cycle
    cmp ax, 0xFFFF
    je end

    mov ah, 0x0e
    int 0x10
    
    jmp printBuffer

end:
    ret

times 510-($-$$) db 0
dw 0xaa55
