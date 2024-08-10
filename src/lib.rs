pub fn hello_world() {
    println!("{:?}", rust_iso3166::ALL_ALPHA2);
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_hello_world() {
        hello_world();
    }
}
