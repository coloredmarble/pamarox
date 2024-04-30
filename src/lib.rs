#![no_std]
#![allow(bad_style)]
#![allow(non_camel_case_types)]
#![allow(internal_features)]
#![feature(rustc_attrs)]
#![feature(lang_items)]

use pipey::Pipey;

pub mod ctype;
pub mod mem;
pub mod playground;
pub mod stdutil;
mod stuff;
pub mod syscall_bind;

#[panic_handler]
#[allow(unconditional_recursion)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    wawa();
    // loopy
    panic(_info);
}

#[lang = "eh_personality"]
fn wawa() {
    unsafe {
        ("\x1b[1;31mwe uhhhhh. fucked up :3\x1b[m\n")
            .pipe(|s| syscall_bind::write(2, s.as_ptr(), s.as_bytes().len()));
        syscall_bind::exit(69);
    }
}

#[no_mangle]
pub fn test_panic() {
    panic!()
}
