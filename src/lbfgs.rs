//lbfgsb binding
use libc::{c_double,c_int,c_char};

extern "C"{
    pub fn setulb_(n:*const c_int,m:*const c_int,x:*mut c_double,l:*const c_double,
    u:*const c_double,nbd:*const c_int,f:*mut c_double,g:*mut c_double,factr:*const c_double,
    pgtol:*const c_double,wa:*mut c_double,iwa:*mut c_int,iprint:*const c_int,csave:*mut c_char,
    lsave:*mut c_int,isave:*mut c_int,dsave:*mut c_double)
}