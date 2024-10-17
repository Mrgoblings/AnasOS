.PHONY: run clean

all: out-folder bootload kernel-rust image clean run

release: out-folder bootload kernel-rust-release image clean run

out-folder:
	-@mkdir out

kernel-rust:
	@cd ./anasos-kernel &&  cargo rustc --target x86_64-unknown-none -- --emit obj
	cp `ls -1 ./anasos-kernel/target/x86_64-unknown-none/debug/deps/anasos_kernel*.o | head -n 1` ./out/kernel.o

kernel-rust-release:
	@cd ./anasos-kernel &&  cargo rustc --release --target x86_64-unknown-none -- --emit obj
	cp `ls -1 ./anasos-kernel/target/x86_64-unknown-none/release/deps/anasos_kernel*.o | head -n 1` ./out/kernel.o


bootload:
	nasm -f elf64 bootloader/boot.asm -o ./out/boot.o

image:
	ld -m elf_x86_64 -T bootloader/linker.ld -o AnasOS/boot/kernel out/boot.o out/kernel.o
	grub-mkrescue -o release/AnasOS.iso AnasOS/

run:
	qemu-system-x86_64 release/AnasOS.iso

clean:
	-@rm -r out
	-@rm -r anasos-kernel/target