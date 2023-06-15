pub fn factorial(n: i32) -> i32 {
    fn factorial_helper(n: i32, acc: i32) -> i32 {
        if n == 0 {
            acc
        } else {
            factorial_helper(n - 1, acc * n)
        }
    }
    factorial_helper(n, 1)
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
