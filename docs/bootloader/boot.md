# Bootloader Deep Dive

This document provides a detailed explanation of the bootloader assembly code that transitions the CPU from 32-bit protected mode to 64-bit long mode. It performs essential checks and configurations to ensure the system is ready for the operating system to take over.

## Table of Contents

1. [Introduction](#introduction)
2. [Stack Setup](#2-stack-setup)
3. [Power-On Self Test (POST)](#3-power-on-self-test-post)
    - [Multiboot Check](#31-multiboot-check)
    - [CPUID Check](#32-cpuid-check)
    - [Long Mode Check](#33-long-mode-check)
4. [Page Table Setup](#4-page-table-setup)
5. [GDT Setup](#5-gdt-setup)
6. [Enabling Paging](#6-enabling-paging)
7. [Error Handling](#7-error-handling)
8. [Conclusion](#conclusion)

## Introduction

A bootloader initializes the system hardware and prepares the environment for the operating system. This specific bootloader sets up the necessary conditions to enable long mode, allowing the CPU to operate in 64-bit mode.

## 2. Stack Setup

Before performing any operations, the bootloader sets up the stack:

```assembly
MOV esp, stack_top  ; Set the stack pointer to the top of the stack
```

- **`ESP` (Extended Stack Pointer)**: Points to the top of the stack. Initializing `ESP` ensures that function calls and interrupts have a valid stack space to work with.

## 3. Power-On Self Test (POST)

The bootloader performs a series of checks to verify the system's capabilities before proceeding.

### 3.1. Multiboot Check

Verifies if the bootloader is loaded by a Multiboot-compliant bootloader:

```assembly
CMP eax, 0x36d76289 ; Compare EAX with the Multiboot magic number
JNE .no_multiboot   ; Jump if not equal to the error handler
```

- **`EAX`**: Contains the magic number passed by the Multiboot bootloader. Comparing it verifies compatibility.
- **`CMP`**: Sets flags based on the comparison between `EAX` and the magic number.
- **`JNE`**: Jumps to `.no_multiboot` if the zero flag is not set, indicating a mismatch.

### 3.2. CPUID Check

Checks if the CPU supports the `CPUID` instruction:

```assembly
PUSHFD              ; Save current EFLAGS to the stack
POP eax             ; Copy EFLAGS into EAX
MOV ecx, eax        ; Store original EFLAGS in ECX
XOR eax, 1 << 21    ; Toggle the ID flag (bit 21)
PUSH eax            ; Push modified EFLAGS back to the stack
POPFD               ; Restore modified EFLAGS
PUSHFD              ; Save modified EFLAGS to stack
POP eax             ; Copy modified EFLAGS into EAX
PUSH ecx            ; Restore original EFLAGS from ECX
POPFD               ; Restore original EFLAGS
CMP eax, ecx        ; Compare modified EFLAGS with original
JE .no_cpuid        ; If equal, CPUID is not supported
```

- **ID Flag (Bit 21 of EFLAGS)**: Indicates support for `CPUID` when it can be modified.
- **`XOR`**: Toggles the specified bit in `EAX`.
- **`PUSHFD`/`POPFD`**: Preserve and restore the EFLAGS register.
- **`CMP` and `JE`**: Determines if the ID flag change was successful.

### 3.3. Long Mode Check

Determines if the CPU supports 64-bit long mode:

```assembly
MOV eax, 0x80000000 ; Set EAX to check extended CPUID functions
CPUID               ; Get highest extended function supported
CMP eax, 0x80000001 ; Check if extended functions are sufficient
JB .no_long_mode    ; Jump if not supported
MOV eax, 0x80000001 ; Get extended processor info
CPUID
TEST edx, 1 << 29   ; Test if Long Mode bit is set in EDX
JZ .no_long_mode    ; Jump if Long Mode is not supported
```

- **Extended Function 0x80000001**: Provides feature bits in `EDX`.
- **Long Mode Bit (Bit 29 of `EDX`)**: Indicates support for 64-bit mode.
- **`TEST`**: Performs bitwise AND without altering operands, sets flags accordingly.
- **`JZ`**: Jumps if the result of `TEST` is zero.

## 4. Page Table Setup

Configures identity-mapped page tables for long mode:

```assembly
MOV eax, page_table_l3    ; Load address of L3 page table
OR eax, 0b11              ; Set Present and Write flags
MOV [page_table_l4], eax  ; Link L4 to L3

MOV eax, page_table_l2    ; Load address of L2 page table
OR eax, 0b11              ; Set flags
MOV [page_table_l3], eax  ; Link L3 to L2

MOV ecx, 0                ; Initialize counter
.loop_setup_page_tables:
MOV eax, 0x200000         ; Load 2MiB page size
MUL ecx                   ; Calculate offset
OR eax, 0b10000011        ; Set Huge Page, Present, and Write flags
MOV [page_table_l2 + ecx * 8], eax

INC ecx
CMP ecx, 512              ; Check if entire table is mapped
JNE .loop_setup_page_tables
```

- **`EAX`**: Used to hold addresses and manipulate page table entries.
- **`OR eax, 0b11`**: Sets the Present and Write flags in the page table entry.
- **`MOV [destination], eax`**: Stores the updated entry in the page table.
- **`ECX`**: Serves as a loop counter for page table entries.
- **`MUL`**: Multiplies `EAX` by `ECX` to compute the memory address for each page.
- **Huge Page**: Indicates a page size of 2MiB.

## 5. GDT Setup

Loads the Global Descriptor Table (GDT) to define the code segment for long mode:

```assembly
LGDT [gdt64.pointer] ; Load the address of the GDT
```

- **`LGDT`**: Loads the GDT register with the address of the GDT.
- **GDT**: Defines memory segments; in long mode, segmentation is largely unused but a minimal GDT is still required.

The GDT is defined in the `.rodata` section:

```assembly
SECTION .rodata
gdt64:
    dq 0 ; null descriptor
.code_segment: EQU $ - gdt64
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53) ; code segment descriptor
.pointer:
    dw $ - gdt64 - 1 ; size of GDT
    dq gdt64         ; address of GDT
```

- **Code Segment Descriptor**: Configured for 64-bit code execution.
- **`EQU`**: Assembler directive to define constants.

## 6. Enabling Paging

Enables paging and prepares the CPU for long mode:

```assembly
; Load the page table address into CR3
MOV eax, page_table_l4
MOV cr3, eax

; Enable Physical Address Extension (PAE)
MOV eax, cr4
OR eax, 1 << 5
MOV cr4, eax

; Enable Long Mode
MOV ecx, 0xC0000080
RDMSR              ; Read from the Model Specific Register
OR eax, 1 << 8
WRMSR              ; Write back to the MSR

; Enable paging
MOV eax, cr0
OR eax, 1 << 31
MOV cr0, eax
```

- **`CR3`**: Control register that holds the address of the page table.
- **PAE (Physical Address Extension)**: Allows 32-bit processors to access more than 4GB of memory.
- **`CR4`**: Used to enable PAE.
- **`RDMSR`/`WRMSR`**: Read and write Model Specific Registers; used here to enable Long Mode.
- **`CR0`**: Control register used to enable paging.

## 7. Error Handling

Handles errors encountered during the boot process:

```assembly
error:
    ; Display "ERR: X" where X is the error code
    MOV dword [0xB8000], 0x4F524F45
    MOV dword [0xB8004], 0x4F3A4F52
    MOV dword [0xB8008], 0x4F204F20
    MOV byte  [0xB800C], al
    HLT
```

- **VGA Text Mode Memory (`0xB8000`)**: Used to display text on the screen.
- **Error Codes**: The error code stored in `AL` is displayed.
- **`HLT`**: Halts the CPU.

## Conclusion

The bootloader meticulously prepares the system for 64-bit operation by performing hardware checks and setting up crucial structures like the stack, page tables, and GDT. It enables paging and long mode before transferring control to the operating system.
