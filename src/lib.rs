#![no_std]
#![allow(bad_style)]
#![allow(non_camel_case_types)]
#![allow(internal_features)]
#![feature(rustc_attrs)]
#![feature(lang_items)]

pub mod ctype;
pub mod stdutil;
pub mod syscall_bind;
pub mod playground;
mod stuff;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // fuck you
    loop {}
}

#[lang = "eh_personality"]
unsafe fn wawa(){
    // my personality is that: when i see a problem. i fucking leave
    syscall_bind::exit(69);
}