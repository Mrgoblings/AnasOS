.PHONY:	run clean

all: out-folder kernel-rust bootload image clean run

release: out-folder kernel-rust-release bootload image clean run


out-folder:
	@([ -d out ] || mkdir out)

kernel-c:
	gcc -m32 -fno-stack-protector -fno-builtin -c tmp-c-kernel/kernel.c -o out/kernel.o

kernel-rust:
	@cd ./anasos-kernel && cargo build --target thumbv7em-none-eabihf
	cp ./anasos-kernel/target/thumbv7em-none-eabihf/debug/anasos-kernel ./out/kernel.o

kernel-rust-release:
	@cd ./anasos-kernel && cargo build --release --target thumbv7em-none-eabihf
	cp ./anasos-kernel/target/thumbv7em-none-eabihf/release/anasos-kernel ./out/kernel.o

bootload:
	nasm -f elf32 bootloader/boot.asm -o ./out/boot.o

image:
	ld -m elf_i386 -T bootloader/linker.ld -o AnasOS/boot/kernel out/boot.o out/kernel.o
	grub-mkrescue -o release/AnasOS.iso AnasOS/

run:
	qemu-system-i386 release/AnasOS.iso

clean:
	rm -r out