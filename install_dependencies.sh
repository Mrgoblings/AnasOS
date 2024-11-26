sudo apt update
sudo apt install -y nasm grub-pc-bin grub-common make mtools xorriso

rustup default nightly-2024-10-14
rustup override set nightly-2024-10-14
rustup target add x86_64-unknown-none --toolchain nightly
rustup component add rust-src --toolchain nightly-2024-10-14-x86_64-unknown-linux-gnu

rustc +nightly --version
