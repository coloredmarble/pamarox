use core::ptr::write_bytes;
pub unsafe extern "C" fn uitoa(mut ptr: *mut u8, n: usize) -> *const u8{
    // fuck fancy formatting
    ptr = ptr.offset(9);
    *ptr.offset(1) = 0;
    while n > 0 {
        *ptr = ('0' as u8) + ((n % 10) as u8);
        ptr = ptr.offset(-1);
    }
    ptr
}