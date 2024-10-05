[org 0x7c00]

; disk nuber is stored in dl
mov [diskNum], dl

; Tiny Memory Model
xor ax, ax
mov ss, ax
mov ds, ax
; mov cs, ax - never change the code segment
mov es, ax

mov ah, 2
mov al, 1
mov ch, 0
mov cl, 2
mov dh, 0
mov dl, [diskNum]
mov bx, 0x7e00
int 0x13

jnc setCarryFlagLow
jc setCarryFlagHigh

push si

cmp al, 1
je setCorrectNOfSegments
jne setWrongNOfSegments

mov ah, 0x0e
mov al, [0x7e00]
; mov al, 'C'
int 0x10

call printSi
pop si
call printSi

jmp $

diskNum: db 0

correctNOfSegments: db "Correct number of Segments.", 0x0D, 0x0A, 0
wrongNOfSegments:   db "Error! Wrong number of Segments.", 0x0D, 0x0A, 0

carryFlagHigh:  db "Error! Carry Number is HIGH", 0x0D, 0x0A, 0
carryFlagLow: db "All good, Carry Number is low", 0x0D, 0x0A, 0

setCorrectNOfSegments:
    mov si, correctNOfSegments
    ret
setWrongNOfSegments:
    mov si, wrongNOfSegments
    ret
setCarryFlagHigh:
    mov si, carryFlagHigh
    ret
setCarryFlagLow:
    mov si, carryFlagLow
    ret

printSi:
    mov al, [si]
    cmp al, 0
    je endPrint

    mov ah, 0x0e
    int 0x10

    inc si
    jmp printSi
endPrint:
    ret

times 510 - ($ - $$) db 0
dw 0xaa55
times 512 db 'A'
