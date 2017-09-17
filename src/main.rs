#![feature(lang_items, unwind_attributes, asm)]
#![no_std]
#![no_main]

#[no_mangle]
pub extern fn main() {
	unsafe {
		let num = 22.625f32;
		if num < 0.0 {
			// Code executes!
			asm!("cli");
			asm!("sleep");
		}
		else {
			asm!("cli");
			asm!("rjmp .-2");
		}
	}
}

pub mod std {
    #[lang = "eh_personality"]
    #[no_mangle]
    pub unsafe extern "C" fn rust_eh_personality(_state: (), _exception_object: *mut (), _context: *mut ()) -> () {}

    #[lang = "panic_fmt"]
    #[unwind]
    pub extern fn rust_begin_panic(_msg: (), _file: &'static str, _line: u32) {}
}
