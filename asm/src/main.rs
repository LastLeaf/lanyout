extern crate lanyout;

#[no_mangle]
pub extern "C" fn test() -> i32 {
    let mut err = 0;
    if lanyout::canvas::test() != 0 { err += 1; }
    return err;
}

fn main() {
    lanyout::init();
    test();
}
