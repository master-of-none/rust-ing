pub fn factorial(n: i32) -> i32 {
    if n == 1 {
        1
    } else if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_factorial() {
        let result = factorial(5);
        assert_eq!(result, 120);
    }
}
