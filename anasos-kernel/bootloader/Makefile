all:
	nasm -f elf64 header.asm -o header.o
	nasm -f elf64 boot-64.asm -o boot-64.o
	nasm -f elf64 boot.asm -o boot.o

	ld -m elf_x86_64 -T ../linker.ld -o ../../AnasOS/boot/kernel boot.o boot-64.o header.o
	grub-mkrescue -o AnasOS.iso ../../AnasOS/
	qemu-system-x86_64 AnasOS.iso