.PHONY: run clean

all: clean out-folder bootload kernel-rust image run

no-run: clean out-folder bootload kernel-rust image

out-folder:
	-@mkdir out

kernel-rust:
	@cd ./anasos-kernel &&  cargo rustc --release --target x86_64-unknown-none -- --emit obj
	@cp `ls -1 ./anasos-kernel/target/x86_64-unknown-none/release/deps/*.o | head -n 1` ./out/kernel.o

bootload:
	@nasm -f elf64 bootloader/boot.asm -o ./out/boot.o

image:
	ld -m elf_x86_64 -T bootloader/linker.ld -o AnasOS/boot/kernel out/boot.o out/kernel.o
	grub-mkrescue -o AnasOS.iso AnasOS/

run:
	qemu-system-x86_64 AnasOS.iso

clean:
	-@rm -r out
	-@rm -r anasos-kernel/target