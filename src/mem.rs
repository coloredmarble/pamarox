use core::mem::{size_of, MaybeUninit};
use core::sync::atomic::AtomicUsize;

use pipey::Pipey;

use crate::stuff::{pstr, very_safe_ptr_inc, void, Either};
use crate::{syscall_bind, unwrap_or_fuck};
// C-like style
// ignore brk errors
// i am a totally resposible unsafe user
// memory is not cleared by alloc. clear it yourself
pub const PAGE_SIZE: usize = 4096;
// also use as heap init point
static mut fbrick_ptr: Option<*mut brickcell> = None;

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
    syscall_bind::brk(0).tap(|a| _ = syscall_bind::brk(a.clone() + l))
}

pub unsafe extern "C" fn init_brick(l: usize) -> *const brickcell {
    let fbrick = pls_giv_moar_memowy(l) as *mut brickcell;
    (*fbrick).size = l;
    // this called with alloc. which means it is already be taken
    (*fbrick).free = false;
    (*fbrick).next = None;
    fbrick
}

pub unsafe fn merge_wall(head: *mut brickcell, tail: *mut brickcell) {
    (*head).size = (tail.addr() + size_of::<brickcell>()) - (head.addr() + size_of::<brickcell>());
    (*head).next = (*tail).next;
}
// this keeps returning marked non-free cells for some reason
// returns Either<merged head, tail offset for insertion>
pub unsafe fn recursive_brick_find_merge(
    head: *mut brickcell,
    tail: *mut brickcell,
    accumlen: usize,
    l: usize,
) -> Either<*mut brickcell, *mut brickcell> {
    if (*tail).free {
        if (*tail).size >= l {
            (*tail).free = false;
            return Either::Left(tail);
        }
        // merge
        if accumlen >= l {
            merge_wall(head, tail);
            (*head).free = false;
            return Either::Left(head);
        }
    }
    match (*tail).next {
        None => Either::Right(tail),
        // tail is not free (fragmentation).
        // find next free chain
        Some(ntail) => recursive_brick_find_merge(ntail, ntail, 0, l),
    }
}
pub unsafe fn shove_brick(tail: *mut brickcell, l: usize) -> *const brickcell {
    let ceiling = syscall_bind::brk(0);
    let nptrloc = tail.add(size_of::<brickcell>() + (*tail).size) as usize;
    if nptrloc >= ceiling {
        // generally. we dont give a fuck
        syscall_bind::brk(PAGE_SIZE + l);
    }
    (nptrloc as *mut brickcell).pipe(|newbptr| {
        // i forgot to add this. i wasted a fucking week on giving up and playing ace combat. it was fire
        (*tail).next = Some(newbptr);
        (*newbptr).size = l;
        // this called with alloc. which means it is already be taken
        (*newbptr).free = false;
        (*newbptr).next = None;
        newbptr
    })
}
pub unsafe fn alloc<T>(len: usize) -> *mut T {
    match fbrick_ptr {
        None => (init_brick(len) as *mut brickcell).tap(|p| fbrick_ptr = Some(p.clone())),
        Some(bk) => match recursive_brick_find_merge(bk, bk, 0, len) {
            // end of brickcell offset
            Either::Left(brickpnt) => brickpnt,
            Either::Right(v) => shove_brick(v, len),
        },
    }
    .add(size_of::<brickcell>()) as *mut T
}
pub unsafe fn mark_free<T>(ptr: *const T) {
    // this will be available when some guy asks for more ram
    let mfcbcell = ptr.sub(size_of::<brickcell>()) as *mut brickcell;
    (*mfcbcell).free = true;
}

// not sure if extern blocks work with generics
#[inline]
#[no_mangle]
pub unsafe extern "C" fn malloc(len: usize) -> *const void {
    alloc(len)
}
#[inline]
#[no_mangle]
pub unsafe extern "C" fn free(ptr: *const void) {
    mark_free(ptr)
}
