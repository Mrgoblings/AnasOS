build:
	nasm -f bin boot.asm -o boot.bin
	qemu-system-x86_64 -drive format=raw,file=boot.bin -vnc :0
	
local:
	nasm -f bin boot.asm -o boot.bin
	qemu-system-x86_64 -drive format=raw,file=boot.bin

