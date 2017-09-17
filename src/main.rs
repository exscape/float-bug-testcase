#![feature(lang_items, unwind_attributes, asm)]
#![no_std]
#![no_main]

extern crate arduino;
use arduino::*;
use core::ptr::write_volatile;

#[no_mangle]
pub extern fn main() {
    small_delay();
    unsafe {
        write_volatile(DDRC, 0b11);
    }

    loop {
        unsafe {
                asm!("sei");
            let num = 22.625f32;
                asm!("cli");
            if num < 0.0 {
                // Code executes!
                asm!("brne .+0");
                asm!("sleep");
                asm!("nop");
            }
            else {
                asm!("breq .+0");
                asm!("sleep");
                asm!("cli");
            }
                asm!("sei");
            large_delay();
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
