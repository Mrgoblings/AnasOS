# AnasOS Documentation

Welcome to the AnasOS documentation. This document will guide you through the various components of the AnasOS operating system, including the multiboot configuration, kernel, window manager, and applications.

## Table of Contents

1. [Introduction](#introduction)
2. [File Structure](#file-structure)
3. [Multiboot Configuration](#multiboot-configuration)
4. [Bootloader](#bootloader)
5. [Kernel](#kernel)
6. [Window Manager](#window-manager)
7. [Applications](#applications)
8. [Technologies Used](#technologies-used)
9. [CI with GitHub Actions](#ci-with-github-actions)
10. [Requirements](#requirements)

## Introduction

AnasOS is a custom operating system designed for educational purposes as part of a 12th-grade thesis project. This documentation provides detailed information about the different parts of the OS and how they work together. AnasOS is built for the **_x86_64_** architecture, chosen over ARM due to the availability of more and better resources on the topic.

## Technologies Used

- **Rust**: The core of the operating system is written in Rust for safety and performance.
- **NASM**: The assembler used for writing assembly code for the bootloader.
- **Makefile**: Used for managing build automation.
- **GRUB**: The bootloader used for loading the kernel.
- **QEMU**: The emulator used for testing the operating system.
- **GitHub Actions**: Used for continuous integration and deployment.

## File Structure

Below is the file structure for the AnasOS documentation:

```
/docs
    ├── README.md
    ├── applications
    │   ├── README.md
    │   ├── browser.md
    │   └── terminal.md
    ├── bootloader
    │   ├── README.md
    │   ├── header.md
    │   ├── boot.md
    │   └── boot-64.md
    ├── ci-github-actions
    │   ├── README.md
    │   ├── header.md
    │   ├── boot.md
    │   └── boot-64.md
    ├── kernel
    │   ├── README.md
    │   └── kernel.md
    ├── multiboot
    │   ├── README.md
    │   └── multiboot.md
    └── window-manager
        ├── README.md
        └── window_manager.md
```

## Multiboot Configuration

The multiboot configuration is responsible for initializing the hardware and loading the kernel into memory. Detailed documentation about the multiboot configuration can be found [here](multiboot/Home.md).

## Bootloader

The bootloader is a critical component that initializes the system and loads the kernel into memory. It is responsible for setting up the environment so that the kernel can execute properly. Detailed documentation about the bootloader can be found [here](bootloader/Home.md).

## Kernel

The kernel is the core component of AnasOS, managing system resources and providing essential services. Detailed documentation about the kernel can be found [here](kernel/Home.md).

## Window Manager

The window manager **WILL** handle the graphical user interface and window management. Detailed documentation about the window manager will be available [here](window-manager/Home.md).

## Applications

AnasOS **WILL** include two main applications: a [browser](applications/browser.md) and a [terminal](applications/terminal.md). Detailed documentation about these applications will be available [here](applications/Home.md).

## CI with GitHub Actions

GitHub Actions is used for continuous integration (CI) to automate the build, testing, and release processes of AnasOS. This ensures that every change is verified, the system remains stable, and new versions are released efficiently. The workflows are defined in the `.github/workflows` directory, with detailed documentation available in the [here](ci-github-actions/Home.md).

## Requirements

Before fully testing the OS, ensure you have the following requirements installed:

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **NASM**: Install NASM from [nasm.us](https://www.nasm.us/).
- **GRUB**: Install GRUB using your package manager.
- **QEMU**: Install QEMU from [qemu.org](https://www.qemu.org/).
- **Make**: Ensure Make is installed on your system.

You can install the required packages on Debian-based Linux distributions using `apt` with the example from the [main Readme](https://github.com/Mrgoblings/AnasOs/blob/main/docs/README.md) file
