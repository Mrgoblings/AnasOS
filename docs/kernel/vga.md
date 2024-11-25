# VGA Module Documentation

## Introduction

The VGA module in the AnasOS kernel is responsible for handling text output to the screen using the VGA text buffer. This module provides functionality to write characters and strings to the screen, manage colors, and handle special characters like newlines and tabs. The implementation leverages Rust's safety features and abstractions to ensure reliable and efficient text output.

This module was created by following [this tutorial](https://os.phil-opp.com/vga-text-mode/).

## Implementation Details

### Color Enum

The `Color` enum defines the various colors that can be used for text and background in the VGA buffer. Each color is represented by a unique 4-bit value.

```rust
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    White = 15
}
```

### ColorCode Struct

The `ColorCode` struct encapsulates the foreground and background colors, as well as additional attributes like blinking. It provides methods to create new color codes and retrieve the individual colors.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}
```

### ScreenChar Struct

The `ScreenChar` struct represents a single character on the screen, including its ASCII value and color code. It provides methods to create new screen characters and blank characters.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(ascii_character: u8, color_code: ColorCode) -> ScreenChar {
        ScreenChar {
            ascii_character,
            color_code,
        }
    }
}
```

### Buffer Struct

The `Buffer` struct represents the VGA text buffer, which is a 2D array of `ScreenChar` instances. It uses the `Volatile` wrapper to prevent the compiler from optimizing away memory accesses.

```rust
use volatile::Volatile;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
```

### Writer Struct

The `Writer` struct is responsible for writing characters and strings to the VGA buffer. It maintains the current cursor position and color code. The `Writer` provides methods to write bytes, handle special characters, scroll the screen, and clear rows.

```rust
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
}
```

### Macros

The `print!` and `println!` macros provide convenient ways to write formatted strings to the VGA buffer. They use the `format_args!` macro from the core library to handle formatting.

```rust
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
```

## Conclusion

The VGA module is a crucial component of the AnasOS kernel, enabling text output to the screen. By leveraging Rust's safety features and abstractions, the module ensures reliable and efficient text handling. The provided enums, structs, and macros make it easy to manage colors, write text, and handle special characters, contributing to a robust and user-friendly kernel.
