use libc::{c_char};

extern "C"{
    pub fn stringfy_(s: *mut c_char);
}