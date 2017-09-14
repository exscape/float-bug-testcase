#![feature(lang_items, unwind_attributes)]
#![no_std]
#![no_main]

extern crate arduino;
use arduino::*;
use core::ptr::{read_volatile, write_volatile};

#[no_mangle]
pub extern fn main() {
    small_delay();
    unsafe {
        write_volatile(DDRC, 0b11);
    }

    loop {
        unsafe {
            let num = 22.625f32;
            if num < 0.0 {
                // Code executes!
                write_volatile(PORTC, 0);
                small_delay();
                write_volatile(PORTC, 0xff);
                small_delay();
            }
            large_delay();
            if 22.625f32 < 0.0 {
                // Code does NOT execute
                // Note to self: stat LED
                for _ in 0..2 {
                    write_volatile(PORTC, 1);
                    small_delay();
                    write_volatile(PORTC, 0xff);
                    small_delay();
                }
            }
            large_delay();
            if 22.625f32 >= 0.0 {
                // Code executes
                // Note to self: net LED
                for _ in 0..4 {
                    write_volatile(PORTC, 2);
                    small_delay();
                    write_volatile(PORTC, 0xff);
                    small_delay();
                }
            }
        }

        large_delay();
        large_delay();
    }
}
#[inline]
fn small_delay() {
    for _ in 0..2500 { }
}
fn large_delay() {
    for _ in 0..10 { small_delay() }
}

pub mod std {
    #[lang = "eh_personality"]
    #[no_mangle]
    pub unsafe extern "C" fn rust_eh_personality(_state: (), _exception_object: *mut (), _context: *mut ()) -> () {}

    #[lang = "panic_fmt"]
    #[unwind]
    pub extern fn rust_begin_panic(_msg: (), _file: &'static str, _line: u32) {}
}
