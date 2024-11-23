# AnasOS

AnasOS is a lightweight operating system developed as a graduation project. The primary goal is to create a functional OS from scratch using modern technologies.

## Download

You can download the latest release of AnasOS, including the `iso.zip` file, from the [releases page](https://github.com/Mrgoblings/AnasOS/releases).

## Technologies Used

- **Rust**: The core of the operating system is written in Rust for safety and performance.
- **Assembly**: Utilized for low-level system programming.
- **Makefile**: Used for managing build automation.

## Building the OS

If you want to build AnasOS yourself, please follow these instructions for a Debian-based Linux distribution:

1. **Clone the repository**:

```sh
git clone https://github.com/Mrgoblings/AnasOS.git
cd AnasOS
```

2. **Install dependencies**:

```sh
sudo apt update
sudo apt install -y nasm grub-pc-bin grub-common make mtools xorriso
rustup update nightly
rustup target add x86_64-unknown-none --toolchain nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```

3. **Build & Run the OS in qemu**:

```sh
make
```

## Documentation

For more detailed instructions and documentation, please refer to the [docs/](docs/) directory. There is a README file that explains everything needed for the OS.

## Author and Licensing

AnasOS is developed by Emil Momchev. The project is licensed under the [MIT License](LICENSE). When distributing, mention the author and the repository.
