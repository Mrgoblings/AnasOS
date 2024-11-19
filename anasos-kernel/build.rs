use std::env;
use std::process::Command;

fn main() {
    let target_dir: String = env::var("OUT_DIR").unwrap(); 

    let boot_o_path: String = format!("{}/boot.o", target_dir);
    let boot_64_o_path: String = format!("{}/boot-64.o", target_dir);
    let header_o_path = format!("{}/header.o", target_dir);


    let status_boot = Command::new("nasm")
        .args(&["-f", "elf64", "bootloader/boot.asm", "-o", &boot_o_path])
        .status()
        .expect("Failed to assemble boot.asm");

    if !status_boot.success() {
        panic!("Assembly of boot.asm failed with status: {}", status_boot);
    }

    let status_boot_64 = Command::new("nasm")
        .args(&["-f", "elf64", "bootloader/boot-64.asm", "-o", &boot_64_o_path])
        .status()
        .expect("Failed to assemble boot-64.asm");

    if !status_boot_64.success() {
        panic!("Assembly of boot.asm failed with status: {}", status_boot_64);
    }

    let status_header = Command::new("nasm")
        .args(&["-f", "elf64", "bootloader/header.asm", "-o", &header_o_path])
        .status()
        .expect("Failed to assemble header.asm");

    if !status_header.success() {
        panic!("Assembly of header.asm failed with status: {}", status_header);
    }


    // Custom linker arguments
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    println!("cargo:rustc-link-arg={}", boot_o_path);
    println!("cargo:rustc-link-arg={}", boot_64_o_path);
    println!("cargo:rustc-link-arg={}", header_o_path);
}
