mod lib_interfaces;

#[no_mangle]
pub extern "C" fn plus(a: i32, b: i32) -> i32 {
    return a + b;
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
