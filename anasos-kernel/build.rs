use std::env;
use std::process::Command;

fn main() {
    // Get the target directory from Cargo's environment
    let target_dir = env::var("OUT_DIR").unwrap(); // OUT_DIR is a directory for build artifacts

    // Define the output path for the bootloader object file
    let bootloader_o_path = format!("{}/bootloader.o", target_dir);

    // Assemble the bootloader
    let status = Command::new("nasm")
        .args(&["-f", "elf64", "boot/bootloader.asm", "-o", &bootloader_o_path])
        .status()
        .expect("Failed to assemble bootloader");

    if !status.success() {
        panic!("Assembly failed with status: {}", status);
    }

    // Specify custom linker arguments
    println!("cargo:rustc-link-arg=-Tlinker.ld"); // Use the correct linker script name
    println!("cargo:rustc-link-arg={}", bootloader_o_path); // Ensure the correct path is used
}
