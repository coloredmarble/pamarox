// used for files. in case you have your heads up your ass
#[repr(C)]
pub struct File{
    pub fd: u32,
    pub flags: u32,
    pub close: fn(*const File)
}
