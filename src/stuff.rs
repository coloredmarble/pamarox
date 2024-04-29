use core::ops::Index;
use pipey::Pipey;

use crate::syscall_bind;
pub type void = ();

#[macro_export]
macro_rules! nil_ptr {
    ($t:ty) => {
        0 as *const $t
    };
}

extern "C" {
    // dear rustc my beloved. you didnt see me deref this *const u8 which may actually be *const boobs cast as *const u8. with love
    pub fn shady_byte_deref(ptr: *const u8) -> u8;
    pub fn shady_ptr_inc(ptr: *const void);
}

pub fn very_safe_ptr_inc<T>(ptr: &mut *const T){
    *ptr = ((*ptr as usize) + 1) as * const T
}

#[no_mangle]
pub extern "C" fn study_mut_refx(x: &mut usize){
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
    pub fn inc(&mut self){
        very_safe_ptr_inc(&mut self.ptr)
    }
    /// current
    pub fn wawa(&self) -> T{
        unsafe {self.ptr.read()}
    }
}

impl<T> Index<usize> for IterPtrTilNil<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*(((self.ptr as usize) + index) as *const T) }
    }
}

impl<T> Iterator for IterPtrTilNil<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            /*let b = shady_byte_deref(self.ptr as *const u8);
            if b == 0 {
                return None;
            } else {
                let x = self.ptr.read();
                self.ptr = self.ptr.offset(1);
                Some(x)
            }*/
            shady_byte_deref(self.ptr as *const u8).pipe(|pp| if 0 == pp {None} else {Some(self.ptr.read().btw(|| self.ptr = self.ptr.offset(1)))})
        }
    }
}

// currently used for debugging
#[no_mangle]
pub unsafe fn pstr(s: *const u8, size: usize){
    syscall_bind::write(1, s, size);
}

#[no_mangle]
pub fn print_str(s: &str){
    unsafe {
        pstr(s.as_ptr(), s.as_bytes().len())
    }
}

// used for files. in case you have your heads up your ass
#[repr(C)]
pub struct File{
    fd: u32,
    flags: u32,
    close: fn(*const File)
}
