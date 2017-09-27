#![no_std]

#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items, start)]
#![feature(unique)]

#[macro_use]

mod mem;
mod stdio;
// mod lib;

extern crate volatile;

// #[no_mangle]
#[inline(never)]
#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize{
    unsafe {
        stdio::clear_screen(stdio::Color::Cyan);
        // stdio::print("hello\nspork", stdio::Color::White, stdio::Color::Black);
        // stdio::print_something();
    }
    1
}


#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
