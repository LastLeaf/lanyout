#[macro_use]
extern crate lazy_static;

mod utils;
mod lib_interfaces;
pub mod canvas;
pub mod frame;

#[no_mangle]
pub extern "C" fn animation_frame(timestamp: f64) {
    frame::generate_frame(timestamp);
}

pub fn init() {
    lib!(init_lib());
}
