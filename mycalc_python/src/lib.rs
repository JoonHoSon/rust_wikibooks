#[no_mangle]
pub extern "C" fn mul(left: isize, right: isize) -> isize {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = mul(10, 20);
        assert_eq!(result, 200);
    }
}
