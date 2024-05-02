pub const MAP_SHARED: i32 = 0x01;
pub const MAP_PRIVATE: i32 = 0x02;
pub const MAP_TYPE: i32 = 0x0f;
pub const MAP_FIXED: i32 = 0x10;
pub const MAP_ANON: i32 = 0x20;
pub const MAP_ANONYMOUS: i32 = MAP_ANON;
pub const MAP_NORESERVE: i32 = 0x4000;
pub const MAP_GROWSDOWN: i32 = 0x0100;
pub const MAP_DENYWRITE: i32 = 0x0800;
pub const MAP_EXECUTABLE: i32 = 0x1000;
pub const MAP_LOCKED: i32 = 0x2000;
pub const MAP_POPULATE: i32 = 0x8000;
pub const MAP_NONBLOCK: i32 = 0x10000;
pub const MAP_STACK: i32 = 0x20000;
pub const MAP_HUGETLB: i32 = 0x40000;
pub const MAP_FILE: i32 = 0;
pub const PROT_NONE: i32 = 0;
pub const PROT_READ: i32 = 1;
pub const PROT_WRITE: i32 = 2;
pub const PROT_EXEC: i32 = 4;
pub const PROT_GROWSDOWN: i32 = 0x01000000;
pub const PROT_GROWSUP: i32 = 0x02000000;
pub const MS_ASYNC: i32 = 1;
pub const MS_INVALIDATE: i32 = 2;
pub const MS_SYNC: i32 = 4;
pub const MCL_CURRENT: i32 = 1;
pub const MCL_FUTURE: i32 = 2;
pub const MCL_ONFAULT: i32 = 4;
pub const POSIX_MADV_NORMAL: i32 = 0;
pub const POSIX_MADV_RANDOM: i32 = 1;
pub const POSIX_MADV_SEQUENTIAL: i32 = 2;
pub const POSIX_MADV_WILLNEED: i32 = 3;
pub const POSIX_MADV_DONTNEED: i32 = 4;