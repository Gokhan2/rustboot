#![no_std]

use core;

pub type physaddr = u32;

macro_rules! physaddr {
	($e:expr) => (($e as physaddr));
}

macro_rules! to_ptr {
	($e:expr) => (($e as *mut u16));
}

pub fn memset(dest: *mut physaddr, src: physaddr, count: u16) {
	for i in 0..count {
		unsafe {
			core::ptr::write_volatile(dest.offset(i as isize), src);
		}
	}
}

pub unsafe fn memcpy(dest: *mut physaddr, src: *const physaddr, count: u16) {
	for i in 0..count {
		core::ptr::write_volatile(dest.offset(i as isize), *src.offset(i as isize));
	}
}