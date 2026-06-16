fn fib(n: i32) -> i32 {
    fn _fib(a: i32) -> i32 {
        match a {
            0 => 0,
            1 => 1,
            b => _fib(b - 2) + _fib(b - 1)
        }
    }

    _fib(n)
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_509::fib;

    #[test]
    fn test_fib_1() {
        assert_eq!(1, fib(2));
    }

    #[test]
    fn test_fib_2() {
        assert_eq!(2, fib(3));
    }

    #[test]
    fn test_fib_3() {
        assert_eq!(3, fib(4));
    }
}
