/*
    The generation of the bootloader configuration file is inspired by here:
    https://github.com/rust-osdev/bootloader/blob/v0.9.25/build.rs
*/

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use toml::Value;

fn assemble_file(input: &str, output: &str) {
    let status = Command::new("nasm")
        .args(&["-f", "elf64", input, "-o", output])
        .status()
        .expect(&format!("Failed to assemble {}", input));

    if !status.success() {
        panic!("Assembly of {} failed with status: {}", input, status);
    }
}

#[derive(Default)]
struct BootloaderConfig {
    physical_memory_offset: Option<u64>,
    kernel_stack_address: Option<u64>,
    kernel_stack_size: Option<u64>,
    boot_info_address: Option<u64>,
}

fn parse_aligned_addr(key: &str, value: &str) -> u64 {
    let num = if value.starts_with("0x") {
        u64::from_str_radix(&value[2..], 16)
    } else {
        u64::from_str_radix(&value, 10)
    }
    .expect(&format!(
        "`{}` in the kernel manifest must be an integer (is `{}`)",
        key, value
    ));

    if num % 0x1000 != 0 {
        panic!(
            "`{}` in the kernel manifest must be aligned to 4KiB (is `{}`)",
            key, value
        );
    }
    num
}

fn parse_to_config(cfg: &mut BootloaderConfig, table: &toml::value::Table) {
    for (key, value) in table {
        match (key.as_str(), value) {
            ("kernel-stack-address", Value::String(s)) => {
                cfg.kernel_stack_address = Some(parse_aligned_addr(key, s));
            }
            ("boot-info-address", Value::String(s)) => {
                cfg.boot_info_address = Some(parse_aligned_addr(key, s));
            }
            ("physical-memory-offset", Value::String(s)) => {
                cfg.physical_memory_offset = Some(parse_aligned_addr(key, s));
            }
            ("kernel-stack-size", Value::Integer(i)) if *i > 0 => {
                cfg.kernel_stack_size = Some(*i as u64);
            }
            (s, _) => panic!("Unknown or invalid key `{}` in kernel manifest", s),
        }
    }
}

fn generate_bootloader_config(out_dir: &str) {
    let config_file_path = format!("{}/bootloader_config.rs", out_dir);
    let mut file = File::create(&config_file_path).expect("Failed to create bootloader_config.rs");

    // Parse and write configuration
    let kernel_manifest = env::var("KERNEL_MANIFEST").expect("KERNEL_MANIFEST not set");
    let contents = fs::read_to_string(&kernel_manifest).expect("Failed to read kernel manifest");
    let manifest = contents
        .parse::<Value>()
        .expect("Failed to parse kernel manifest");

    let mut config = BootloaderConfig::default();
    if let Some(table) = manifest
        .get("package")
        .and_then(|pkg| pkg.get("metadata"))
        .and_then(|meta| meta.get("bootloader"))
        .and_then(Value::as_table)
    {
        parse_to_config(&mut config, table);
    }

    // Here, you can hardcode or dynamically calculate configuration values.
    let config_content = format!(
        "const PHYSICAL_MEMORY_OFFSET: Option<u64> = {:?};
const KERNEL_STACK_ADDRESS: Option<u64> = {:?};
const KERNEL_STACK_SIZE: u64 = {};
const BOOT_INFO_ADDRESS: Option<u64> = {:?};",
        config.physical_memory_offset,
        config.kernel_stack_address,
        config.kernel_stack_size.unwrap_or(512),
        config.boot_info_address
    );

    file.write_all(config_content.as_bytes())
        .expect("Failed to write to bootloader_config.rs");
}

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
    generate_bootloader_config(&target_dir);

    // Custom linker arguments
    println!("cargo:rustc-link-search=native={}", target_dir);
    println!("cargo:rustc-link-arg=-Tlinker.ld");
}
