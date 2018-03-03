extern {
    fn sample_callback(arg: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn plus(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    println!("Hello world!");
    unsafe {
        sample_callback(0x1000);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        assert_eq!(super::main(), ());
    }
}
