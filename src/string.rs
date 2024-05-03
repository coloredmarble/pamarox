use core::ptr::{copy_nonoverlapping, write_bytes};

use crate::stuff::void;
/* 
#[no_mangle]
pub unsafe extern "C" fn memcpy(dst: *mut void,src: *const void, len: usize) -> *const void{
    copy_nonoverlapping(src, dst, len);
    dst
}
#[no_mangle]
pub unsafe extern "C" fn memset(str: *mut void,val: u8, len: usize) -> *const void{
    write_bytes(str, val, len);
    str
}*/

extern "C"{
    // inefficient but works
    pub fn memcpy(dst: *mut void,src: *const void, len: usize) -> *const void;
    pub fn memset(str: *mut void,val: u8, len: usize) -> *const void;
    pub fn strlen(str: *const u8) -> usize;
}