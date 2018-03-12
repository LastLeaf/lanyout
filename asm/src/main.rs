extern crate lanyout;

#[no_mangle]
pub extern "C" fn test() -> i32 {
    let mut err = 0;
    if lanyout::canvas::test::test() != 0 { err += 1; }
    if lanyout::canvas::animation::test::test() != 0 { err += 1; }
    return err;
}

fn main() {
    lanyout::init();
    test();
}
