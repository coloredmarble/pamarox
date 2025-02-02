use core::{mem::size_of, ops::Index};
use pipey::Pipey;

use crate::{string, syscall_bind};
pub type void = ();

// https://stackoverflow.com/questions/29963449/golang-like-defer-in-rust
struct ClosureDefer<F: FnOnce()> {
    c: Option<F>,
}
impl<F: FnOnce()> Drop for ClosureDefer<F> {
    fn drop(&mut self) {
        self.c.take().unwrap()()
    }
}
macro_rules! expr {
    ($e: expr) => {
        $e
    };
} // tt hack
#[macro_export]
macro_rules! defer {
    ($($data: tt)*) => (
        let _scope_call = ClosureDefer {
            c: || -> () { expr!({ $($data)* }) }
        };
    )
}

#[macro_export]
macro_rules! unwrap_or_fuck {
    ($opt:expr,$f:tt) => {
        match $opt{
            Some(v) => v,
            None => $f
        }
    };
}

extern "C" {
    pub fn shady_byte_deref(ptr: *const u8) -> u8;
    pub fn shady_ptr_inc(ptr: *const void);
}

pub fn very_safe_ptr_inc<T>(ptr: &mut *const T) {
    *ptr = (ptr.addr() + size_of::<T>()) as *const T
}

#[no_mangle]
pub extern "C" fn study_mut_refx(x: &mut usize) {
    *x += 1;
}

/// same size as *const T
/// created for iter over const char* but keep flexibility there incase i realize how dumb it is
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IterPtrTilNil<T> {
    pub ptr: *const T,
}

impl<T> IterPtrTilNil<T> {
    pub fn new(ptr: *const T) -> Self {
        IterPtrTilNil { ptr }
    }
    pub fn inc(&mut self) {
        very_safe_ptr_inc(&mut self.ptr)
    }
    /// current
    pub fn wawa(&self) -> T {
        unsafe { self.ptr.read() }
    }
}

impl<T> Index<usize> for IterPtrTilNil<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.ptr.add(index) }
    }
}

impl<T> Iterator for IterPtrTilNil<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            shady_byte_deref(self.ptr as *const u8).pipe(|pp| {
                if 0 == pp {
                    None
                } else {
                    Some(self.ptr.read().btw(|| self.ptr = self.ptr.offset(1)))
                }
            })
        }
    }
}

// currently used for debugging
#[no_mangle]
pub unsafe fn pstr(s: *const u8, size: usize) {
    syscall_bind::write(1, s, size);
}

#[no_mangle]
// byte by byte
pub unsafe fn pstr_til_nil(str: *const u8) {
    syscall_bind::write(1, str, str.addr() + string::strlen(str));
}
 
pub enum Either<L,R>{
    Left(L),
    Right(R),
}