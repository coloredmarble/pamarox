use core::mem::MaybeUninit;

use crate::consts::mmap::*;
use crate::stuff::{pstr, very_safe_ptr_add, very_safe_ptr_inc, void};
use crate::syscall_bind;

// idk how to get page size at runtime. got it from my pooter
pub const PAGE_SIZE: usize = 4096;
static mut fbrick_uninit: MaybeUninit<*const brickcell> = core::mem::MaybeUninit::uninit();
// static mut start_brick: brickcell;

// https://www.youtube.com/watch?v=CulF4YQt6zA
#[repr(C)]
pub struct brickcell {
    pub size: usize,
    pub free: bool,
    pub next: *mut brickcell,
}

// slab-based
#[no_mangle]
pub unsafe extern "C" fn shit_bricks(mut bricksizes: *const usize, mut n: usize) {
    let saddr = syscall_bind::brk(0);
    let fcellptr: *const brickcell = saddr as _;
    let mut current_cell = fcellptr as *mut brickcell;
    while n > 0 {
        // i have 2 options
        // 1. get sum of (bricksizes + sizeof(brickcell)) * n (2 iterations)
        // 2. do it in 1 go but with alot of brks
        // i dont fucking wawa
        let csize = bricksizes.read();
        syscall_bind::brk(saddr + core::mem::size_of::<brickcell>() + csize);
        (*current_cell).size = csize;
        (*current_cell).free = true;
        n -= 1;
        if n == 0 {
            break;
        }
        very_safe_ptr_inc(&mut bricksizes);
        (*current_cell).next = (current_cell.offset(1).addr() + csize) as *mut brickcell;
        current_cell = (*current_cell).next;

    }
    fbrick_uninit.as_mut_ptr().write(fcellptr);
}

#[no_mangle]
pub unsafe extern "C" fn shit_alloc(len: usize) -> *const void{
    let mut fbrickcell = fbrick_uninit.assume_init();
    loop {
        if (*fbrickcell).free && (*fbrickcell).size >= len{
            // give end of struct
            return fbrickcell.offset(1) as *const void;
        }
        if (*fbrickcell).next.is_null(){
            break;
        }
        fbrickcell = (*fbrickcell).next
    }
    0 as *const void
}
