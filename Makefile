boot:
	[ -d out ] || mkdir out
	gcc -m32 -fno-stack-protector -fno-builtin -c tmp-c-kernel/kernel.c -o out/kernel.o
	nasm -f elf32 bootloader/boot.asm -o out/boot.o
	ld -m elf_i386 -T bootloader/linker.ld -o AnasOS/boot/kernel out/boot.o out/kernel.o
	rm -r out
	
	grub-mkrescue -o release/AnasOS.iso AnasOS/
	qemu-system-i386 release/AnasOS.iso
