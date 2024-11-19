.PHONY: run clean

all: clean kernel-rust image run

vnc: clean kernel-rust image run-vnc

no-run: clean kernel-rust image

test: no-run
	echo "Compiled the OS successfully"

kernel-rust:
	@cd ./anasos-kernel && cargo build --release

image:
	@cp  ./anasos-kernel/target/x86_64-unknown-none/release/anasos-kernel AnasOS/boot/kernel
	grub-mkrescue -o AnasOS.iso AnasOS/

run:
	qemu-system-x86_64 AnasOS.iso

run-vnc:
	qemu-system-x86_64 AnasOS.iso -vnc :0

clean:
	-@cd ./anasos-kernel && cargo clean > /dev/null 2>&1
	-@rm AnasOS/boot/kernel > /dev/null 2>&1
	-@rm AnasOS.iso > /dev/null 2>&1
