/*
    This file is inspired by:
    https://github.com/rust-osdev/bootloader/blob/v0.9.25/build.rs
*/

use std::env;
use std::fs::File;
use std::io::Write;
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

// fn generate_bootloader_config(out_dir: &str) {
//     let config_file_path = format!("{}/bootloader_config.rs", out_dir);
//     let mut file = File::create(&config_file_path).expect("Failed to create bootloader_config.rs");

//     // Hardcoded configuration values.
//     let config_content = r#"
// const PHYSICAL_MEMORY_OFFSET: Option<u64> = Some(0x100000);
// const KERNEL_STACK_ADDRESS: Option<u64> = Some(0x200000);
// const KERNEL_STACK_SIZE: u64 = 512; // size in pages
// const BOOT_INFO_ADDRESS: Option<u64> = Some(0x300000);
// "#;

//     file.write_all(config_content.as_bytes())
//         .expect("Failed to write to bootloader_config.rs");
// }

fn main() {
    let target_dir = env::var("OUT_DIR").unwrap();

    // Assemble ASM files
    let asm_files = ["boot.asm", "boot-64.asm", "header.asm"];
    for file in &asm_files {
        let input_path = format!("bootloader/{}", file);
        let output_path = format!("{}/{}.o", target_dir, file);

        assemble_file(&input_path, &output_path);
        println!("cargo:rustc-link-arg={}", output_path);
    }

    // Generate bootloader configuration file
    // generate_bootloader_config(&target_dir);

    // Custom linker arguments
    println!("cargo:rustc-link-arg=-Tlinker.ld");
}
