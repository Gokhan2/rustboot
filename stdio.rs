// #![no_std]

#[macro_use]

use core;
use mem;

// use core::ptr::write_volatile;

use mem::physaddr;

const VGABASE: physaddr = physaddr!(0xb8000);

const VGA_X: u8 = 80;
const VGA_Y: u8 = 24;

static mut console_x: u8 = 0;
static mut console_y: u8 = 0;

#[derive(Copy, Clone)]
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

pub fn putc(c: u8, fg: Color, bg: Color) {
	unsafe {
	core::ptr::write_volatile(to_ptr!(VGABASE).offset((console_x + (console_y * VGA_X)) as isize),
		to_vga(c, fg, bg)
		)
	}
}

pub fn putchar(c: u8, fg: Color, bg: Color) {
	putc(c, fg, bg);
	unsafe {
	console_x += 1;
	if console_x >= VGA_X {
		console_x = 0;
		console_y += 1;
		if console_y >= VGA_Y {
			console_y = 0;
		}
	}
}
}

pub fn print(s: &str, fg: Color, bg: Color) {
	for c in s.bytes() {
		putchar(c, fg, bg);
	}
}