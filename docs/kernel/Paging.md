# Paging setup for the bootloader

source: https://devblogs.microsoft.com/oldnewthing/20160701-00/?p=93785

Paging structure is hierarchical. To describe a page of memory, you first select a page directory pointer from the page directory pointer table (PDPT). That points to a page directory (PD). You then select a then a page directory entry (PDE) from the page directory. This points to a page table (PT). You then select a page table entry (PTE) from the page table. The page table entry tells you where the memory for the page can be found.
