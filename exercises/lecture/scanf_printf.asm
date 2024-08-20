;======Plan======
; 1. Print a string
; 2. Get a number from the user
; 3. Print the number back

;

[org 0x7c00]

; Tiny Memory Model
xor ax, ax ; cool way to set ax to 0. XOR with itself is always 0
mov ss, ax
mov ds, ax
    
call ourAmazingFunction
jmp $   
    
defString: db "Enter a number smaller than 254:", 0x0D, 0x0A, 0
    
    
ourAmazingFunction:
    mov si, defString
    
printingLoop:
    mov al, [si]
    cmp al, 0
    je getNumber
    
    ; mov al, [si]
    mov ah, 0x0e
    int 0x10
     
    inc si
    jmp printingLoop

getNumber:
    ; counter for 3 digits
    mov dl, 3

    ; multiplicant
    mov dh, 10

    ; define cl
    mov cl, 0

numberLoop:
    ; check end of loop
    cmp dl, 0
    jbe printNumber

    dec dl
   
    ; get char (no error detection implemented)
    mov ah, 0
    int 0x16

    ; check for an enter character '\r'
    cmp al, 0x0D
    je printNumber

    ; print back the digit:
    mov ah, 0x0e
    int 0x10

    mov ch, al

    ; multiply cl by 10 (room for a new digit)
    mov al, cl
    mul dh ; multiply by 10
    mov cl, al

    ; make to digit
    sub ch, '0'

    add cl, ch

    jmp numberLoop

printNumber:
    ; clear the line from before
    mov ah, 0x0e
    mov al, 0x0D
    int 0x10
    mov al, 0x0A
    int 0x10

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
    
    ; save for next iteration 
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
