build: kernel bootloader
	
bootloader:
	nasm -f bin boot.asm -o boot.bin
	qemu-system-x86_64 -drive format=raw,file=boot.bin

bootloader-vnc:
	nasm -f bin boot.asm -o boot.bin
	qemu-system-x86_64 -drive format=raw,file=boot.bin -vnc :0

kernel:
	@cd ./anasos-kernel && cargo build --target thumbv7em-none-eabihf

kernel-release: 
	@cd ./anasos-kernel && cargo build --release --target thumbv7em-none-eabihf

