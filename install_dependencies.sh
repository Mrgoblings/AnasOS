sudo apt update
sudo apt install -y nasm grub-pc-bin grub-common make mtools xorriso
rustup override set nightly-2024-10-14
rustup target add x86_64-unknown-none --toolchain nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
