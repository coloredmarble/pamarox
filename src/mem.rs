use core::mem::{size_of, MaybeUninit};

use pipey::Pipey;

use crate::stuff::{pstr, very_safe_ptr_inc, void, Either};
use crate::{syscall_bind, unwrap_or_fuck};
// C-like style
// ignore brk errors
// idk how to get page size at runtime. got it from my pooter
pub const PAGE_SIZE: usize = 4096;
static mut fbrick_ptr: Option<*mut brickcell> = None;
// static mut start_brick: brickcell;

// https://www.youtube.com/watch?v=CulF4YQt6zA
pub struct brickcell {
    pub size: usize,
    pub free: bool,
    pub next: Option<*mut brickcell>,
}

// sbrk
/// extends current program heap end by l. or by whatever the fuck the kernel wants
#[no_mangle]
pub unsafe extern "C" fn pls_giv_moar_memowy(l: usize) -> usize {
    syscall_bind::brk(0).pipe(|a| syscall_bind::brk(a + l))
}

pub unsafe extern "C" fn init_bricks(l: usize) {
    let fbrick = pls_giv_moar_memowy(l) as *mut brickcell;
    (*fbrick).size = l;
    // this called with alloc. which means it is already be taken
    (*fbrick).free = false;
    (*fbrick).next = None;
    fbrick_ptr = Some(fbrick)
}

// returns Either<merged head, cbrick for insertion update>
pub unsafe fn recursive_brick_find_merge(
    head: *mut brickcell,
    cbrick: *mut brickcell,
    accumlen: usize,
    l: usize,
) -> Either<*const brickcell, *mut brickcell> {
    if (*cbrick).free {
        if (*cbrick).size >= l {
            return Either::Left(cbrick);
        }
        // merge
        if accumlen >= l {
            (*cbrick).free = false;
            if (*cbrick).next.is_some() {
                (*head).next = (*cbrick).next
            } else {
                (*head).next = None
            }
            (*head).size = accumlen;
            (*head).free = false;
            return Either::Left(head);
        }
    }
    // no brick too big. no dick too small
    if (*cbrick).next.is_none() {
        return Either::Right(cbrick);
    }
    // probably overflows stack if we go too far. its a feature
    (*cbrick).next.unwrap_unchecked().pipe(|nbrick| {
        recursive_brick_find_merge(head, nbrick, accumlen + (nbrick.addr() - cbrick.addr()), l)
    })
}

pub unsafe fn shove_brick(tail: *mut brickcell, l: usize) -> *const brickcell {
    let ceiling = syscall_bind::brk(0);
    let nptrloc = tail.offset(1).add((*tail).size) as usize;
    if nptrloc >= ceiling {
        // generally. we dont give a fuck
        syscall_bind::brk(PAGE_SIZE + l);
    }
    let newbptr = nptrloc as *mut brickcell;
    (*newbptr).size = l;
    // this called with alloc. which means it is already be taken
    (*newbptr).free = false;
    (*newbptr).next = None;
    newbptr
}

pub unsafe fn alloc<T>(len: usize) -> *mut T {
    if fbrick_ptr.is_none() {
        init_bricks(len);
        return fbrick_ptr.unwrap().offset(1) as *mut T;
    }
    (fbrick_ptr.unwrap()).pipe(|fbrick_unwrap| {
        match recursive_brick_find_merge(fbrick_unwrap, fbrick_unwrap, 0, len) {
            // end of brickcell offset
            Either::Left(brick) => return brick,
            Either::Right(v) => shove_brick(v, size_of::<T>()),
        }
    }).offset(1) as *mut T
}

pub unsafe fn mark_free<T>(ptr: *const T) {
    // this will be available when some guy asks for more ram
    let cbcell = ptr.sub(size_of::<brickcell>()) as *mut brickcell;
    (*cbcell).free = true;
}

#[no_mangle]
pub unsafe extern "C" fn malloc(len: usize) -> *const void {
    alloc(len)
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *const void) {
    mark_free(ptr)
}
