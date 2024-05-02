use crate::stuff::void;
use core::arch::asm;
// amd64 linux
// too lazy to know how to import unistd.h in my rust
macro_rules! syscall {
    ($code:expr) => (
        // usually isize
        let mut _r;
        asm!("syscall" , in("rax") $code, lateout("rax") _r);
        return _r;
    );
}

macro_rules! syscall_no_ret {
    ($code:expr) => {
        asm!("syscall" , in("rax") $code)
    };
}
// sys/unistd likely already have this. but am too lazy to import it
// linux syscalls probably uses sysv ABI
// extern "C" probably also does
// so the arguments are already in place for the syscall

#[no_mangle]
pub unsafe extern "C" fn exit(_code: i32) {
    syscall_no_ret!(60)
}

#[no_mangle]
pub unsafe extern "C" fn read(_fd: u32, _buf: *mut u8, _size: usize) -> usize {
    syscall!(0);
}

#[no_mangle]
pub unsafe extern "C" fn write(_fd: u32, _buf: *const u8, _size: usize) -> usize {
    syscall!(1);
}

#[no_mangle]
pub unsafe extern "C" fn open(_filename: *const u8, _flags: u32, _mode: u32) -> usize {
    syscall!(2);
}

#[no_mangle]
pub unsafe extern "C" fn close(_fd: u32) -> usize {
    syscall!(3);
}

// fuck off_t
#[no_mangle]
pub unsafe extern "C" fn mmap(
    _addr: *const void,
    _len: usize,
    _prot: i32,
    _flags: i32,
    _fd: i32,
    _off: u64,
) -> *const void {
    syscall!(9);
}

#[no_mangle]
pub unsafe extern "C" fn brk(_addr: usize) -> usize{
    syscall!(12);
} 

