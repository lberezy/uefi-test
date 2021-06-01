use core::ffi::c_void;

// type EfiHandle = *const usize;
#[derive(Clone, Copy, Debug)]
#[repr(transparent)] // this is just a type wrapper struct
pub struct Handle(*mut c_void);
