#![no_std]
#![allow(bad_style)]
#![allow(non_camel_case_types)]
#![allow(internal_features)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(panic_info_message)]
#![feature(strict_provenance)]
#![feature(let_chains)]
#![feature(unwrap_infallible)]
#![feature(ptr_sub_ptr)]

use pipey::Pipey;

pub mod consts;
pub mod ctype;
pub mod mem;
pub mod playground;
pub mod stdutil;
pub mod string;
mod stuff;
pub mod syscall_bind;
#[panic_handler]
#[allow(unconditional_recursion)]
// use panicinfo after i can format
fn panic(info: &core::panic::PanicInfo) -> ! {
    wawa();
    // loopy
    panic(info);
}

#[lang = "eh_personality"]
fn wawa() {
    unsafe {
        "\n\n\x1b[1;31mwe uhhhhh. fucked up :3\x1b[m\n"
            .pipe(|s| syscall_bind::write(2, s.as_ptr(), s.as_bytes().len()));
        syscall_bind::exit(69)
    }
}

#[no_mangle]
pub fn test_panic() {
    panic!()
}
