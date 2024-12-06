sudo apt update
sudo apt install -y nasm grub-pc-bin grub-common make mtools xorriso build-essential cmake libsdl2-dev libsdl2-mixer-dev

rustup install nightly-2024-10-14
rustup default nightly-2024-10-14
rustup override set nightly-2024-10-14
rustup target add x86_64-unknown-none --toolchain nightly
rustup component add rust-src --toolchain nightly-2024-10-14-x86_64-unknown-linux-gnu

rustc --version
