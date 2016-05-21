extern crate lbfgsb_sys;
#[test]
fn timer(){
    use lbfgsb_sys::timer as ffi;
    let mut time  = 0.0;
    unsafe{
        ffi::timer_(&mut time);
    }
    println!("{}",time);
}