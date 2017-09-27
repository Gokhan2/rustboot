
// #[macro_use]

use core;
// use mem;

use mem::physaddr;
use core::ptr::Unique;

const VGABASE: physaddr = physaddr!(0xb8000);

const VGA_X: usize = 80;
const VGA_Y: usize = 25;
const VGASIZE: isize = (VGA_X as isize * VGA_Y as isize);

// static mut console_x: u8 = 0;
// static mut console_y: u8 = 0;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

// #[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct VGAChar {
    character: u8,
    color_code: ColorCode,
}

struct Buffer {
    chars: [[VGAChar; VGA_X]; VGA_Y],
}

pub struct Writer {
    col: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        if self.col >= VGA_X {
            self.new_line();
        }

        let row = VGA_Y - 1;
        self.buffer().chars[row][self.col] = VGAChar {
            character: byte,
            color_code: self.color_code
        };
        self.col = self.col +  1;
    }
    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.as_mut() }
    }
    fn new_line(&mut self) {
        for y in 1..VGA_Y {
            for x in 0..VGA_X {
                self.buffer().chars[y][x] = self.buffer().chars[y - 1][x];
            }
        }
    }
}




pub fn clear_screen(color: Color) {
    unsafe {
        for i in 0..(80 * 25) {
            core::ptr::write_volatile((VGABASE as *mut u16).offset(i), ((color as u16) << 12))
        }
    }
}

pub fn to_vga(c: u8, fg: Color, bg: Color) -> u16 {
	return ((fg as u16) << 8) | (c as u16);
}

pub fn print_something() {
    let mut writer = Writer {
        col: 0,
        color_code: ColorCode::new(Color::LightGreen, Color::Black),
        buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
    };

    writer.write_byte(b'H');
}

// pub fn putc(c: u8, fg: Color, bg: Color) {
// 	unsafe {
// 	core::ptr::write_volatile(to_ptr!(VGABASE).offset((console_x + (console_y * VGA_X)) as isize),
// 		to_vga(c, fg, bg)
// 		)
// 	}
// }

// pub fn putchar(c: u8, fg: Color, bg: Color) {
//     if c != '\n' as u8 {
//         putc(c, fg, bg);
//     }
// 	unsafe {
// 	console_x += 1;
// 	if console_x >= VGA_X || c == '\n' as u8 {
// 		console_x = 0;
// 		console_y += 1;
// 		if console_y >= VGA_Y {
// 			console_y = 0;
// 		}
// 	}
// }
// }

// pub fn print(s: &str, fg: Color, bg: Color) {
// 	for c in s.bytes() {
// 		putchar(c, fg, bg);
// 	}
// }

// // pub fn printf(s: &str, 