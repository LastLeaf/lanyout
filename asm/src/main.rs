mod lib_interfaces;
mod canvas;

use canvas::Canvas;

#[no_mangle]
pub extern "C" fn test() -> i32 {
    let mut canvas = Canvas::new(0);
    canvas.set_clear_color(0., 1., 1., 0.5);
    canvas.clear();
    return 0;
}

fn main() {
    unsafe {
        lib_interfaces::init_lib();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        assert_eq!(super::main(), ());
    }
}
