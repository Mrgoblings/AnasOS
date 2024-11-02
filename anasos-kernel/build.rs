use std::env;
use std::process::Command;

fn main() {
    let target_dir: String = env::var("OUT_DIR").unwrap(); 

    let bootloader_o_path: String = format!("{}/bootloader.o", target_dir);

    let status: std::process::ExitStatus = Command::new("nasm")
        .args(&["-f", "elf64", "boot/bootloader.asm", "-o", &bootloader_o_path])
        .status()
        .expect("Failed to assemble bootloader");

    if !status.success() {
        panic!("Assembly failed with status: {}", status);
    }

    // Custom linker arguments
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    println!("cargo:rustc-link-arg={}", bootloader_o_path);
}
