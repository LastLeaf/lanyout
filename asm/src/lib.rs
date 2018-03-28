#![feature(unsize, coerce_unsized)]

#[macro_use]
extern crate lazy_static;

mod utils;
mod lib_interfaces;
pub mod ctx;
pub mod frame;
pub mod canvas;

#[no_mangle]
pub extern "C" fn callback(callback_ptr: *mut lib_interfaces::Callback, ret_code: i32) {
    let mut callback = unsafe { Box::from_raw(callback_ptr) };
    callback.callback(ret_code);
}

#[no_mangle]
pub extern "C" fn animation_frame(timestamp: f64) {
    frame::generate(timestamp);
}

pub fn init() {
    lib!(init_lib());
}

pub fn main_loop() {
    lib!(emscripten_exit_with_live_runtime());
}
