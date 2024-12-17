use std::env;
use std::process::Command;

fn assemble_file(input: &str, output: &str) {
    let status = Command::new("nasm")
        .args(&["-f", "elf64", input, "-o", output])
        .status()
        .expect(&format!("Failed to assemble {}", input));

    if !status.success() {
        panic!("Assembly of {} failed with status: {}", input, status);
    }
}

fn main() {
    let target_dir: String = env::var("OUT_DIR").unwrap(); 

    let boot_o_path: String = format!("{}/boot.o", target_dir);
    let boot_64_o_path: String = format!("{}/boot-64.o", target_dir);
    let header_o_path = format!("{}/header.o", target_dir);

    assemble_file("bootloader/boot.asm", &boot_o_path);
    assemble_file("bootloader/boot-64.asm", &boot_64_o_path);
    assemble_file("bootloader/header.asm", &header_o_path);

    // Custom linker arguments
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    println!("cargo:rustc-link-arg={}", boot_o_path);
    println!("cargo:rustc-link-arg={}", boot_64_o_path);
    println!("cargo:rustc-link-arg={}", header_o_path);
}
