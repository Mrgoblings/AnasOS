.PHONY: run clean

all: clean kernel-rust image run

vnc: clean kernel-rust image run-vnc

no-run: clean kernel-rust image

test:
	echo "No tests for now"

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
	-@rm -r anasos-kernel/target
	-@rm AnasOS/boot/kernel
	-@rm AnasOS.iso
