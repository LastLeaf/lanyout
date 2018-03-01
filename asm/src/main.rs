fn main() {
    println!("Hello world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        assert_eq!(super::main(), ());
    }
}
