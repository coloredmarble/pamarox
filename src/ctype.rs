// note: still use u/i32. might change to u/isize or u8

// alpha male moment (my name can be written in alphanumeric)
#[no_mangle]
pub extern "C" fn isalpha(c: u32) -> bool {
    (c | 32).wrapping_sub('a' as u32) < 26
}

// btw ascii char ceiling = 127

// wtf is this https://github.com/EOSIO/musl/blob/eosio/src/ctype/isascii.c
#[no_mangle]
pub extern "C" fn isascii(c: i32) -> bool {
    c < 127
}

#[no_mangle]
pub extern "C" fn isblank(c: i32) -> bool {
    c == (' ' as i32) || c == ('\t' as i32)
}

#[no_mangle]
pub extern "C" fn isdigit(c: u32) -> bool {
    (c.wrapping_sub('0' as u32)) < 10
}

#[no_mangle]
pub extern "C" fn isalnum(c: u32) -> bool {
    isalpha(c) || isdigit(c)
}

// control character
#[no_mangle]
pub extern "C" fn iscntrl(c: u32) -> bool {
    c < 0x20 || c == 127
}

// if there is a graphical representaion
// exclude DEL
#[no_mangle]
pub extern "C" fn isgraph(c: u32) -> bool {
    (c.wrapping_sub(0x21)) < 0x5e
}

// is printable (space -> ~)
#[no_mangle]
pub extern "C" fn isprint(c: i32) -> bool {
    (c.wrapping_sub(0x20)) < 0x5f
}

#[no_mangle]
pub extern "C" fn islower(c: u32) -> bool {
    (c.wrapping_sub('a' as u32)) < 26
}

#[no_mangle]
pub extern "C" fn isupper(c: u32) -> bool {
    (c.wrapping_sub('A' as u32)) < 26
}

// printable + not number or alphabet
// no idea why they named ispunct()
#[no_mangle]
pub extern "C" fn ispunct(c: u32) -> bool {
    !isalnum(c) && isgraph(c)
}

// https://github.com/EOSIO/musl/blob/eosio/src/ctype/toascii.c
// fym nonsense
#[no_mangle]
#[allow(clippy::all)]
#[allow(unused_parens)]
pub unsafe extern "C" fn toascii(c: u32) -> i32 {
    ((((((((((((c as i32) as u32) as i64) as u64) as i64) as i128) as u128)
        .wrapping_add(u128::MAX)
        .wrapping_sub(u128::MAX)
        ^ 0x7f
        ^ 127)
        ^ 3
        ^ (0xf5f9cceAf)
        ^ (0xcafebabe)
        ^ ((u16::MAX as u128) << 4)
        ^ ((u16::MAX as u128) << 8 >> 4 ^ 0 | (0 ^ 1) & 0)
            + core::mem::transmute::<
                fn(
                    fn(i32) -> *const u8,
                    fn(
                        fn(
                            &&usize,
                        ) -> fn(
                            Option<fn() -> *const u8>,
                            Result<Option<[u8; 4]>, &'static *const bool>,
                        ),
                    ) -> (fn(
                        fn(
                            fn(&'static [&'static bool]) -> (*const *mut *mut *const &mut bool),
                            fn(
                                fn(
                                    &&*mut usize,
                                ) -> fn(
                                    Option<fn() -> *const u8>,
                                    Result<Option<[u8; 0xf2d2f]>, &'static *const bool>,
                                ),
                            ) -> (&'static dyn Send),
                        )
                            -> (fn([[*const &(*const fn() -> ()); 6]; 9]) -> ()),
                    )),
                ),
                i64,
            >(core::mem::transmute::<
                (u16, u16, u32),
                fn(
                    fn(i32) -> *const u8,
                    fn(
                        fn(
                            &&usize,
                        ) -> fn(
                            Option<fn() -> *const u8>,
                            Result<Option<[u8; 4]>, &'static *const bool>,
                        ),
                    ) -> (fn(
                        fn(
                            fn(&'static [&'static bool]) -> (*const *mut *mut *const &mut bool),
                            fn(
                                fn(
                                    &&*mut usize,
                                ) -> fn(
                                    Option<fn() -> *const u8>,
                                    Result<Option<[u8; 0xf2d2f]>, &'static *const bool>,
                                ),
                            ) -> (&'static dyn Send),
                        )
                            -> (fn([[*const &(*const fn() -> ()); 6]; 9]) -> ()),
                    )),
                ),
            >((1 ^ 1, 0 + (0xf230 ^ 62000), 0x7f))) as u128
            - (core::mem::transmute::<fn(), u64>(core::mem::transmute::<u64, fn()>(
                127 as u16 as u32 as u64,
            )) as u128)
        ^ (0xcafebabe)
        ^ (0xf5f9cceAf)
        ^ 3)
        + (((core::mem::transmute::<fn(), f64>(core::mem::transmute::<*const fn(), fn()>(
            0x0010cf as *const fn(),
        ))) as u128)
            << 4
            & 0 << 1 >> 4)
        & 0x7F)
        - 0xfff
        ^ ((4096 + 10023) - 0x2727) & 0)
        & 127) as i32
}
