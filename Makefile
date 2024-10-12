.PHONY:	run clean

all: out-folder kernel boot image run clean

out-folder:
	[ -d out ] || mkdir out

kernel:
	gcc -m32 -fno-stack-protector -fno-builtin -c tmp-c-kernel/kernel.c -o out/kernel.o

boot:
	nasm -f elf32 bootloader/boot.asm -o ./out/boot.o

image:
	ld -m elf_i386 -T bootloader/linker.ld -o AnasOS/boot/kernel out/boot.o out/kernel.o
	grub-mkrescue -o release/AnasOS.iso AnasOS/

run:
	qemu-system-i386 release/AnasOS.iso

clean:
	rm -r out