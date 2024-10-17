.PHONY: run clean

all: out-folder bootload kernel-rust image clean run

release: out-folder bootload kernel-rust-release image clean run

out-folder:
	@([ -d out ] || mkdir out)

kernel-c:
	gcc -m32 -fno-stack-protector -fno-builtin -c tmp-c-kernel/kernel.c -o out/kernel_c.o

# kernel-rust:
# 	@cd ./anasos-kernel && cargo build --target thumbv7em-none-eabihf
# 	cp ./anasos-kernel/target/thumbv7em-none-eabihf/debug/libanasos_kernel.a ./out/kernel.a
# 	# @ar x ./out/kernel.a

# kernel-rust-release:
# 	@cd ./anasos-kernel && cargo build --release --target x86_64-unknown-none.json
# 	cp ./anasos-kernel/target/x86_64-unknown-none.json/release/libanasos_kernel.a ./out/kernel.a
# 	# ar x ./out/kernel.a
# 	mv *.o ./out/

kernel-rust:
	# @cd ./anasos-kernel && cargo build  --target x86_64-unknown-none
	# cp ./anasos-kernel/target/x86_64-unknown-none/debug/anasos-kernel ./out/kernel.o
	@cd ./anasos-kernel &&  cargo rustc --target x86_64-unknown-none -- --emit obj
	cp `ls -1 ./anasos-kernel/target/x86_64-unknown-none/debug/deps/anasos_kernel*.o | head -n 1` ./out/kernel.o

kernel-rust-release:
	# @cd ./anasos-kernel && cargo build --release --target x86_64-unknown-none
	# cp ./anasos-kernel/target/x86_64-unknown-none/release/anasos-kernel ./out/kernel.o
	@cd ./anasos-kernel &&  cargo rustc --release --target x86_64-unknown-none -- --emit obj
	cp `ls -1 ./anasos-kernel/target/x86_64-unknown-none/debug/deps/anasos_kernel*.o | head -n 1` ./out/kernel.o


bootload:
	nasm -f elf64 bootloader/boot.asm -o ./out/boot.o

image:
	ld -m elf_x86_64 -T bootloader/linker.ld -o AnasOS/boot/kernel out/boot.o out/kernel.o
	grub-mkrescue -o release/AnasOS.iso AnasOS/

run:
	qemu-system-x86_64 release/AnasOS.iso

clean:
	rm -r out