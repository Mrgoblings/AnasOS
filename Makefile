.PHONY: run clean

all: clean kernel-rust image run

vnc: clean kernel-rust image run-vnc

no-run: clean kernel-rust image

test: clean
	# @cd ./anasos-kernel && cargo build --release
	@cd ./anasos-kernel && cargo test --release
	echo "Success"

cargo-run: 
	@cp  ./target/x86_64-unknown-none/release/anasos-kernel ../AnasOS/boot/kernel
	grub-mkrescue -o ../AnasOS.iso ../AnasOS/
	qemu-system-x86_64 ../AnasOS.iso


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
	-@cd ./anasos-kernel && cargo clean > /dev/null 2> /dev/null
	-rm AnasOS/boot/kernel > /dev/null 2> /dev/null
	-rm AnasOS.iso > /dev/null 2> /dev/null
