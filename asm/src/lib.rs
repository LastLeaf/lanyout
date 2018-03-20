#[macro_use]
extern crate lazy_static;

mod utils;
mod lib_interfaces;
pub mod ctx;
pub mod frame;
pub mod canvas;

#[no_mangle]
pub extern "C" fn animation_frame(timestamp: f64) {
    frame::generate(timestamp);
}

pub fn init() {
    lib!(init_lib());
}
