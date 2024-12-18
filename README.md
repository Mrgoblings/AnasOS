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

2. **Install dependencies in `Debian` based distros with the `apt` packet manager**:

```sh
./install_dependencies.sh
```

3. **Build & Run the OS in qemu**:

```sh
make
```

## Documentation

For more detailed instructions and documentation, please refer to the [docs/](docs/) directory. There is a README file that explains everything needed for the OS.

## References

Here are some tutorials and resources that were used in the creation of AnasOS:

- [ Writing an OS in Rust from Philipp Oppermann's blog](https://os.phil-opp.com/)
- [rust-osdev bootloader crate](https://github.com/rust-osdev/bootloader/blob/v0.9.25)
- [Write Your Own 64-bit Operating System Kernel by CodePulse](https://www.youtube.com/playlist?list=PLZQftyCk7_SeZRitx5MjBKzTtvk0pHMtp)
- [Making an OS (x86) by Daedalus Community](https://www.youtube.com/playlist?list=PLm3B56ql_akNcvH8vvJRYOc7TbYhRs19M)
- [Operating Systems by OliveStem](https://www.youtube.com/playlist?list=PL2EF13wm-hWAglI8rRbdsCPq_wRpYvQQy)
- [Stack Unwinding](https://www.bogotobogo.com/cplusplus/stackunwinding.php)
- [Rust Standard Library Runtime](https://github.com/rust-lang/rust/blob/bb4d1491466d8239a7a5fd68bd605e3276e97afb/src/libstd/rt.rs#L32-L73)
- [Name Mangling](https://en.wikipedia.org/wiki/Name_mangling)
- [Calling Convention](https://en.wikipedia.org/wiki/Calling_convention)
- [Cross Compilation with Clang](https://clang.llvm.org/docs/CrossCompilation.html#target-triple)
- [Multiboot Specification](https://wiki.osdev.org/Multiboot)
- [GNU GRUB Multiboot](https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#OS-image-format)
- [Paging in Operating System](https://www.geeksforgeeks.org/paging-in-operating-system/)

## Author and Licensing

AnasOS is developed by Emil Momchev. The project is licensed under the [MIT License](LICENSE). When distributing, mention the author and the repository.
