# Paging setup for the bootloader

source: https://devblogs.microsoft.com/oldnewthing/20160701-00/?p=93785

Paging structure is hierarchical. To describe a page of memory, you first select a page directory pointer from the page directory pointer table (PDPT). That points to a page directory (PD). You then select a then a page directory entry (PDE) from the page directory. This points to a page table (PT). You then select a page table entry (PTE) from the page table. The page table entry tells you where the memory for the page can be found.




### Understanding the x86_64 Paging Code

Paging is a memory management feature that maps virtual addresses to physical addresses. It is essential for modern operating systems, especially when enabling long mode (64-bit). This code sets up a basic paging structure in assembly for an x86_64 processor. Here’s an in-depth explanation:

---

#### **Core Concepts of Paging**

1. **Page Tables**:
   - **Page Table Levels**: x86_64 uses a 4-level hierarchy:
     - **PML4 (Page Map Level 4 Table)**: Top-level table, points to PDPTE (Page Directory Pointer Table Entries).
     - **PDPTE (Page Directory Pointer Table Entry)**: Points to PDEs (Page Directory Entries).
     - **PDE (Page Directory Entry)**: Points to PT (Page Tables) or maps directly to 2MiB pages (as in this code).
     - **PT (Page Table)**: Maps 4KiB pages (not used in this example as huge pages are used).

2. **Memory Addresses**:
   - Virtual addresses are split into parts for indexing each table level:
     - PML4 (bits 39–47), PDPTE (bits 30–38), PDE (bits 21–29), PT (bits 12–20).

3. **Key Registers**:
   - **`CR3`**: Holds the base address of the PML4 table.
   - **`CR4`**: Controls processor features; enables PAE.
   - **`CR0`**: Enables paging.

4. **Attributes**:
   - Entries in tables include flags for present (`P`), writable (`RW`), huge page (`PS`), etc.

---

### Code Walkthrough

#### 1. **Multiboot and CPU Checks**
Before setting up paging, the system checks:
- **Multiboot compatibility**: Ensures the bootloader complies with Multiboot standards.
- **CPUID support**: Verifies the CPU supports the CPUID instruction.
- **Long Mode support**: Ensures the CPU supports 64-bit operation and paging.

#### 2. **Setting Up Page Tables**

##### **Overview**
This code sets up 3 levels of page tables (PML4, PDPTE, PDE) with identity mapping:
- Virtual addresses map directly to physical addresses.

##### **Step-by-Step Explanation**

1. **Setup PML4 Table**:
   - `MOV eax, page_table_l3`: Load address of the next table (L3).
   - `OR eax, 0b11`: Mark entry as present and writable.
   - `MOV [page_table_l4], eax`: Store this entry in the PML4 table.

2. **Setup PDPTE Table**:
   - `MOV eax, page_table_l2`: Load address of L2 table.
   - `OR eax, 0b11`: Mark entry as present and writable.
   - `MOV [page_table_l3], eax`: Store this entry in the PDPTE table.

3. **Setup PDE Table**:
   - Initialize `ecx` (counter) to 0.
   - In the loop:
     - Compute the physical address of each 2MiB page: `MUL ecx` multiplies `0x200000` (2MiB) by `ecx`.
     - Add flags (`OR eax, 0b10000011`): Mark as a huge page, present, and writable.
     - Store the entry in the PDE table: `MOV [page_table_l2 + ecx * 8], eax`.
   - Repeat until `ecx = 512` (entire table mapped).

#### 3. **Enabling Paging**

1. **Load PML4 Table Base**:
   - `MOV eax, page_table_l4`: Load the PML4 table address.
   - `MOV cr3, eax`: Set `CR3` to the PML4 base address.

2. **Enable PAE (Physical Address Extension)**:
   - `MOV eax, cr4`: Load `CR4`.
   - `OR eax, 1 << 5`: Set the PAE bit.
   - `MOV cr4, eax`: Write back to `CR4`.

3. **Enable Long Mode**:
   - Use `RDMSR`/`WRMSR` to modify the `EFER` MSR (Model Specific Register).
   - `OR eax, 1 << 8`: Set the Long Mode Enable (LME) bit.

4. **Enable Paging**:
   - `MOV eax, cr0`: Load `CR0`.
   - `OR eax, 1 << 31`: Set the paging enable bit.
   - `MOV cr0, eax`: Write back to `CR0`.

---

### Adding a 4th Table (PT)

To extend this setup to use 4KiB pages, modify the PDE loop to set up PTs:
1. Allocate a PT for each PDE entry.
2. Map each PT entry to 4KiB pages.

Example:
```asm
MOV ecx, 0 ; counter for PDE
.loop_pde:
    ; Allocate a page table for this PDE
    MOV eax, PT_address
    ADD eax, ecx * 4096 ; Offset for this PT
    OR eax, 0b11 ; Present, writable
    MOV [page_table_l2 + ecx * 8], eax ; Add PT to PDE

    ; Setup the PT
    MOV edx, 0 ; counter for PT entries
.loop_pt:
    MOV eax, 0x1000 ; 4KiB
    MUL edx
    OR eax, 0b11 ; Present, writable
    MOV [PT_address + ecx * 4096 + edx * 8], eax

    INC edx
    CMP edx, 512 ; PT entries
    JNE .loop_pt

    INC ecx
    CMP ecx, 512 ; PDE entries
    JNE .loop_pde
```

---

### Checking System Memory

1. Use the BIOS-provided memory map (via `INT 0x15, E820` or Multiboot info).
2. Read MSRs like `IA32_MTRR_DEF_TYPE` to determine memory regions.

---

### Recommendations for Improved Implementation

1. **Dynamic Table Allocation**:
   - Use memory detection to dynamically allocate tables.
   - Maintain a free list of physical pages.

2. **Modular Flags**:
   - Create macros or constants for entry flags for readability.

3. **Error Handling**:
   - Check for memory allocation failures.
   - Add fallback for non-2MiB page support.

---

### Registers Used
- `eax`: General-purpose, holds addresses and flags.
- `ecx`: Counter for loops.
- `cr3`: Base address of PML4.
- `cr4`: Enables PAE.
- `cr0`: Enables paging.

---

### Self-Implementation Checklist
1. **Memory Layout**: Determine physical memory layout.
2. **Allocate Tables**: Reserve aligned memory for PML4, PDPTE, PDE, and optionally PTs.
3. **Map Virtual to Physical**: Fill table entries with correct mappings and flags.
4. **Enable Paging**: Update `CR3`, set PAE in `CR4`, enable paging in `CR0`.

Let me know if you need help refining any part of this explanation or writing the extended code!