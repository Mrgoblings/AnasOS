#![no_std]
#![no_main]

// implement me the 4 level paging in a way to easily call it from the bootloader 

use x86_64::structures::paging::page_table::PageTable;
use x86_64::structures::paging::PageTableFlags;
use x86_64::structures::paging::Mapper;
use x86_64::structures::paging::OffsetPageTable;
use x86_64::structures::paging::Page;
use x86_64::structures::paging::PhysFrame;
use x86_64::structures::paging::Size4KiB;
use x86_64::VirtAddr;
use x86_64::PhysAddr;

use bootloader::BootInfo;

