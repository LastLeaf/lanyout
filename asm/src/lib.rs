#![feature(unsize, coerce_unsized)]

#[macro_use]
extern crate lazy_static;

mod utils;
mod lib_interfaces;
pub mod ctx;
pub mod frame;
pub mod canvas;

use std::any::Any;

#[no_mangle]
pub extern "C" fn callback(callback_ptr: *mut (), ret_code: i32) {
    let mut callback: Box<lib_interfaces::Callback> = unsafe { (*(callback_ptr as *mut Box<Any>)).downcast().unwrap() };
    callback.callback(ret_code); // TODO dynamic dispatch failed?
}

#[no_mangle]
pub extern "C" fn animation_frame(timestamp: f64) {
    frame::generate(timestamp);
}

pub fn init() {
    lib!(init_lib());
    test();
}

pub fn main_loop() {
    lib!(emscripten_exit_with_live_runtime());
}

// test
struct CustomCb (i32);
impl lib_interfaces::Callback for CustomCb {
    fn callback(&mut self, time: i32) {
        println!("Date.now: {}", time);
    }
}
fn test() {
    lib!(timeout(1000, lib_interfaces::register_callback(Box::new(CustomCb(10)))));
}
