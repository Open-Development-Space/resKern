#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

/// Colors
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
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Build a color code struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

/// Define creation of a new color code
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// What a character on screen should look like
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// What a ScreenWriter should look like
use core::fmt;
pub struct ScreenWriter {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

// Implement functions necessary for a screen writer
impl ScreenWriter {
    /// Write a byte to the screen
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
    }

    /// Write a string / series of bytes to the screen
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not in printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Write a new line
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clear a row from the screen
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

/// Formatted writes to screen
impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref SCREENWRITER: Mutex<ScreenWriter> = Mutex::new(ScreenWriter {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SCREENWRITER.lock().write_fmt(args).unwrap();
}

#[doc(hidden)]
pub fn _print_color(args: fmt::Arguments, color: Color) {
    use core::fmt::Write;

    let mut writer = SCREENWRITER.lock();
    let original_color_code = writer.color_code;
    writer.color_code = ColorCode((original_color_code.0 & 0xf0) | color as u8);
    let result = writer.write_fmt(args);
    writer.color_code = original_color_code;
    result.unwrap();
}

/// Print given input to screen
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

/// Print given input in the specified foreground color to screen.
#[macro_export]
macro_rules! print_cl {
    ($color:expr, $($arg:tt)*) => {
        $crate::vga_buffer::_print_color(format_args!($($arg)*), $color)
    };
}

/// Print given input with a new line at end to screen.
#[macro_export]
macro_rules! print_ln {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Print given input in the specified foreground color with a new line at end.
#[macro_export]
macro_rules! print_ln_cl {
    ($color:expr) => {
        $crate::print_cl!($color, "\n")
    };
    ($color:expr, $($arg:tt)*) => {
        $crate::vga_buffer::_print_color(format_args!("{}\n", format_args!($($arg)*)), $color)
    };
}
