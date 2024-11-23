# BOOT-64.ASM Documentation

## Introduction
This document provides a detailed explanation of the `BOOT-64.ASM` file, which is responsible for initializing the CPU in 64-bit long mode. After jumping into 64-bit mode, it's essential to set up the CPU environment correctly to ensure proper operation. The code is written in assembly language and is crucial for setting up the environment before the operating system kernel takes over. This documentation aims to explain each instruction and its purpose.

## Code Explanation

### Section: Setting Up Long Mode
```assembly
GLOBAL long_mode_start 
EXTERN _start

SECTION .text
BITS 64
long_mode_start:
    ; load null to all data segment registers (needed for the cpu to work as intended)
    MOV ax, 0
    MOV ss, ax
    MOV ds, ax
    MOV es, ax
    MOV fs, ax
    MOV gs, ax

    CALL _start

    HLT
```

#### Detailed Breakdown

1. **GLOBAL long_mode_start**
   - Makes the `long_mode_start` label available to other files, allowing them to reference this entry point.

2. **EXTERN _start**
   - Declares an external symbol `_start`, which is the main function of the Rust kernel. This is where the kernel's execution begins.

3. **SECTION .text**
   - Defines the beginning of a code section named `.text`, where the executable code resides.

4. **BITS 64**
   - Informs the assembler that the following code is intended for 64-bit mode.

5. **long_mode_start:**
   - A label marking the entry point of the long mode setup code. At this point, the system has just transitioned into 64-bit mode.

6. **MOV ax, 0**
   - Loads the value `0` into the `AX` register to prepare for initializing the segment registers.

7. **MOV ss, ax**
   - Sets the `SS` (Stack Segment) register to `0`. In 64-bit mode, all segment registers need to be set to `0` for the CPU to function properly, ensuring correct memory addressing.

8. **MOV ds, ax**
   - Sets the `DS` (Data Segment) register to `0`, which is necessary for proper data access.

9. **MOV es, ax**
   - Sets the `ES` (Extra Segment) register to `0`.

10. **MOV fs, ax**
    - Sets the `FS` (Additional Data Segment) register to `0`.

11. **MOV gs, ax**
    - Sets the `GS` (Additional Data Segment) register to `0`. Initializing all these registers prevents segmentation errors and ensures the CPU operates as expected.

12. **CALL _start**
    - Calls the `_start` function, transferring control to the Rust kernel's main function.

13. **HLT**
    - Halts the CPU, typically used if there is nothing else to execute.

## Conclusion
The `BOOT-64.ASM` file is a critical component in the boot process of an operating system. After entering 64-bit long mode, setting all segment registers to `0` is essential for the CPU to function properly. This setup prepares the environment for the kernel's main function `_start` to take over. Each instruction serves a specific purpose to ensure the CPU operates correctly and transitions smoothly to the kernel.
