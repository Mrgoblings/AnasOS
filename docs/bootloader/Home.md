# AnasOS Bootloader Documentation

## Overview

The AnasOS bootloader is a critical component of the AnasOS operating system. Written in NASM assembly, it is responsible for initializing the system and transitioning the CPU from real mode to 64-bit long mode. This document provides an overview of the bootloader's functionality and its role in the boot process.

## Boot Files

The bootloader consists of three main files:
- **header.asm**: Contains the Multiboot2 header required for bootloader compatibility. More in the [header.md](header.md)
- **boot.asm**: The main bootloader code, responsible for the entire boot process including setting up the stack, checking Multiboot compatibility, enabling paging, and handling errors More in the [boot.md](boot.md)
- **boot-64.asm**: Handles the transition to 64-bit long mode. More in the [boot-64.md](boot-64.md)

## Integration with the Kernel

The bootloader is tightly integrated with the AnasOS kernel. The build process is managed by a `build.rs` script, which automates the compilation and linking of the bootloader and kernel object files into a single binary.

### Build Process

The `build.rs` script performs the following steps:
1. **Assemble Bootloader**: The NASM assembler is used to compile the bootloader assembly files into object files.
    ```sh
    nasm -f elf64 header.asm -o header.o
    nasm -f elf64 boot.asm -o boot.o
    nasm -f elf64 boot-64.asm -o boot-64.o
    ```
2. **Link Object Files**: The object files are linked together with the kernel object files to create a single binary.
    ```sh
    ld -n -o bootloader.bin -T linker.ld header.o boot.o boot-64.o kernel.o
    ```

### Bootloader Compilation

To compile only the bootloader manually, you would typically run the following commands:
```sh
nasm -f elf64 header.asm -o header.o
nasm -f elf64 boot.asm -o boot.o
nasm -f elf64 boot-64.asm -o boot-64.o
ld -n -o bootloader.bin -T linker.ld header.o boot.o boot-64.o
```

For simplicity, there is a Makefile in the `anasos-kernel/bootloader` folder that handles this compilation. This Makefile is a simplified version of the main Makefile from the root folder, but its purpose is to compile correctly and run the newly created ISO from the bootloader with QEMU.

## Conclusion

The AnasOS bootloader is a sophisticated piece of software that ensures a smooth transition from the initial power-on state to a fully operational 64-bit environment. Its careful design and implementation in NASM assembly make it a reliable foundation for the AnasOS operating system.
