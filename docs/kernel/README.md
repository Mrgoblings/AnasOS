# AnasOS Kernel

Welcome to the AnasOS Kernel documentation. This kernel is written in Rust and is designed as a thesis project.

## Overview

The AnasOS Kernel is currently in development and includes the following features:
- VGA text mode support (see [vga.md](vga.md) for more in-depth documentation)
- A `build.rs` script to bundle the kernel with the bootloader

## Upcoming Features

I am actively working on adding support for:
- Programmable Interrupt Timer (PIT)
- I/O interrupts
- PS/2 keyboard driver

## Building the Kernel

To build the kernel, you will need to have Rust installed on your system. You can then use the following commands to build and run the kernel:
