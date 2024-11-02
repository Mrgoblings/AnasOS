.PHONY: run clean

all: clean kernel-rust image run

no-run: clean kernel-rust image

test:
	echo "No tests for now"

kernel-rust:
	@cd ./anasos-kernel && cargo +nightly build --release --target x86_64-unknown-none

image:
	@cp  ./anasos-kernel/target/x86_64-unknown-none/release/anasos-kernel AnasOS/boot/kernel
	grub-mkrescue -o AnasOS.iso AnasOS/

run:
	qemu-system-x86_64 AnasOS.iso

clean:
	-@rm -r anasos-kernel/target
