compile:
	nasm -f elf64 header.asm -o header.o
	nasm -f elf64 boot-64.asm -o boot-64.o
	nasm -f elf64 boot.asm -o boot.o
	nasm -f elf64 e820.asm -o e820.o

	ld -m elf_x86_64 -T ../linker.ld -o ../../AnasOS/boot/kernel boot.o boot-64.o header.o e820.o
	grub-mkrescue -o AnasOS.iso ../../AnasOS/
	
run: compile
	qemu-system-x86_64 AnasOS.iso

debug: compile
	qemu-system-x86_64 AnasOS.iso -s -S

debug-gdb:
	gdb ../../AnasOS/boot/kernel -q -ex "target remote localhost:1234" 