use core::arch::asm;
// amd64 linux
// too lazy to know how to import unistd.h in my rust
macro_rules! syscall {
    ($code:expr) => {
        // usually isize
        let mut _r = 0;
        asm!("syscall" , inlateout("rax") $code => _r);
        return _r;
    };
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
pub unsafe extern "C" fn exit(_code: i32){
    syscall_no_ret!(60)
}

#[no_mangle]
pub unsafe extern "C" fn read(_fd: u32, _buf: *mut u8, _size: usize) -> usize{
    syscall!(0usize);
}

#[no_mangle]
pub unsafe extern "C" fn write(_fd: u32, _buf: *const u8, _size: usize) -> usize{
    syscall!(1usize);
}

#[no_mangle]
pub unsafe extern "C" fn open(_filename: *const u8, _flags: u32, _mode: u32) -> usize{
    syscall!(2usize);
}

#[no_mangle]
pub unsafe extern "C" fn close(_fd: u32) -> usize{
    syscall!(3usize);
}