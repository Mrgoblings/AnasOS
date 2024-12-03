# Multiboot Configuration for AnasOS

This document explains the Multiboot configuration file used to boot AnasOS.

## Introduction

The [Multiboot specification](https://en.wikipedia.org/wiki/Multiboot_specification) is an open standard that defines how bootloaders can load x86 operating system kernels. This standard ensures compatibility between bootloaders and operating system kernels, simplifying the boot process. By adhering to the Multiboot specification, developers can create and maintain operating systems more efficiently.

## Why Choose Multiboot Over Custom Bootloaders?

Opting for Multiboot instead of developing a custom bootloader offers several benefits:
- **Standardization**: Multiboot provides a consistent interface for bootloaders and kernels, reducing development complexity.
- **Flexibility**: It supports various filesystems and kernel formats, making it adaptable to different environments.
- **Community Support**: Widely used and well-documented, Multiboot offers extensive resources and community assistance.

## Multiboot1 vs Multiboot2

[Multiboot2](https://en.wikipedia.org/wiki/Multiboot_specification) is an enhanced version of the original Multiboot specification (Multiboot1). Key improvements include:
- **Extended Information**: Provides detailed information about the boot environment, such as memory maps and boot modules.
- **64-bit Support**: Designed to support 64-bit long mode, essential for modern 64-bit operating systems like AnasOS.
- **Modularity**: Allows for more modular and flexible boot configurations.

## GRUB Multiboot2 Setup

AnasOS uses GRUB Multiboot2 for booting. The GRUB configuration file is located at `AnasOS/boot/grub/grub.cfg`.

- `set timeout=5`
    - Sets the boot menu timeout to 5 seconds.

- `menuentry "AnasOS" {`
    - Defines a menu entry named "AnasOS".

- `multiboot2 /boot/kernel`
    - Specifies the kernel file path to be loaded using Multiboot2.

- `boot`
    - Instructs the bootloader to boot the selected operating system.

## Multiboot Magic Number

The Multiboot2 specification uses a unique "magic number" (0x36d76289) to identify Multiboot-compliant kernels. This magic number is checked by the bootloader to ensure compatibility. In AnasOS, the bootloader incorporates this check to verify that the kernel adheres to the Multiboot2 standard, ensuring a smooth and standardized boot process.

## References

- [Multiboot Specification](https://en.wikipedia.org/wiki/Multiboot_specification)
