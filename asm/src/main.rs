extern crate lanyout;

#[no_mangle]
pub extern "C" fn test() -> i32 {
    let mut err = 0;
    err += lanyout::frame::test::test();
    err += lanyout::canvas::test::test();
    return err;
}

fn main() {
    lanyout::init();
    test();
    lanyout::main_loop();
}
