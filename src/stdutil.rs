use pipey::Pipey;

// stdlib.c
use crate::stuff::{print_str, shady_byte_deref, shady_ptr_inc, void, IterPtrTilNil};

// return non-negative number
#[no_mangle]
pub extern "C" fn abs(n: i32) -> i32 {
    if 0 < n {
        return n;
    }
    return -n;
}

// get 4 bits only (dont give a fuck about other bits)
// https://rust-malaysia.github.io/code/2020/07/11/faster-integer-parsing.html
// optimize later
#[no_mangle]
pub extern "C" fn atoi(mut iptr: IterPtrTilNil<u8>) -> isize {
    let ine = iptr[0] == ('-' as u8);
    if ine {
        iptr.inc()
    }
    iptr.fold(0, |n, c| {
        (10 * n).btw(|| print_str("1")) - (c & 0x0f) as isize
    })
    .pipe(|n| if ine { n } else { -n })
}
