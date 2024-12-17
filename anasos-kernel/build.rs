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

    let asm_files = [
        ("bootloader/boot.asm", "boot.o"),
        ("bootloader/boot-64.asm", "boot-64.o"),
        ("bootloader/header.asm", "header.o"),
    ];

    for (input, output) in &asm_files {
        let output_path = format!("{}/{}", target_dir, output);
        assemble_file(input, &output_path);
        println!("cargo:rustc-link-arg={}", output_path);
    }
    
    // Custom linker arguments
    println!("cargo:rustc-link-arg=-Tlinker.ld");
}
