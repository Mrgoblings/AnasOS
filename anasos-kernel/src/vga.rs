// Used to remove Rust optimizations on the Buffer struct
use volatile::Volatile;

// Used to create a static mutable Writer instance
use lazy_static::lazy_static;

// This allows us to use the printing traits from the core library
use core::fmt;

// Used to create a mutex for the Writer instance
use spin::Mutex;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(
        Writer::new_default()
    );
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


// Number	    Color	    Number + Bright Bit	Bright Color
// 0b0000	    Black	    0x1000	Dark Gray
// 0x0001	    Blue	    0x1001	Light Blue
// 0x0010	    Green	    0x1010	Light Green
// 0x0011	    Cyan	    0x1011	Light Cyan
// 0x0100	    Red	        0x1100	Light Red
// 0x0101	    Magenta	    0x1101	Pink
// 0x0110	    Brown	    0x1110	Yellow
// 0x0111	    Light Gray	0x1111	White
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}


// [1]            [111]       [1]        [111]         [1111 1111]
// is_blinking    back_color  is_bright  fore_color    char code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

#[allow(dead_code)]
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    fn new_blinking(foreground: Color, background: Color) -> ColorCode {
        let mut color_code = Self::new(foreground, background).0;

        color_code |= 0x80; // Set the highest bit for blinking

        ColorCode(color_code)
    }

    fn get_colors(&self) -> (Color, Color) {
        let color_code = self.0;
        let foreground = color_code & 0x0F;
        let background = (color_code & 0xF0) >> 4;

        (unsafe { core::mem::transmute(foreground) }, unsafe { core::mem::transmute(background) })
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    fn new(ascii_character: u8, color_code: ColorCode) -> ScreenChar {
        ScreenChar {
            ascii_character,
            color_code,
        }
    }

    fn blank(color_code: ColorCode) -> ScreenChar {
        ScreenChar::new(b' ', color_code)
    }
}


const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    line_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

#[allow(dead_code)]
impl Writer {
    pub fn new_default() -> Writer {
        Writer {
            column_position: 0,
            line_position: 0,
            color_code: ColorCode::new(Color::LightGray, Color::Black),
            buffer: unsafe { &mut *(VGA_BUFFER as *mut Buffer) },
        }
    }

    pub fn new_with_colors(foreground: Color, background: Color) -> Writer {
        Writer {
            column_position: 0,
            line_position: 0,
            color_code: ColorCode::new(foreground, background),
            buffer: unsafe { &mut *(VGA_BUFFER as *mut Buffer) },
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
    
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => self.column_position = 0, // TODO ask - no self.clear_row(self.line_position) here?,
            b'\t' => {
                const TAB_SIZE: usize = 4;
                let spaces = TAB_SIZE - (self.column_position % TAB_SIZE);
                for _ in 0..spaces {
                    self.write_byte(b' ');
                }
            }
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                // check if not a valid ASCII character to print "■"
                let current_char: ScreenChar;
                match byte {
                    0x20..=0x7e => current_char = ScreenChar::new(byte, self.color_code),
                    _ => current_char = ScreenChar::new(0xfe, self.color_code),
                }

                self.buffer.chars[self.line_position][self.column_position].write(current_char);

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        if self.line_position < BUFFER_HEIGHT - 1 {
            self.line_position += 1;
        } else {
            self.scroll_up();
        }

        self.column_position = 0;
    }

    fn scroll_up(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::blank(self.color_code);

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }

        self.column_position = 0;
    }

    fn reset(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }

        self.line_position = 0;
        // self.column_position = 0; - in the clear_row function
    }
}



#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
