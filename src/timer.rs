use libc::{c_double};

extern "C"{
    pub fn timer_(d: *mut c_double);
}