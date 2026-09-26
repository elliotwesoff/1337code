fn tribonacci(n: i32) -> i32 {
    match n {
        0 => return 0,
        1 | 2 => return 1,
        _ => (),
    }

    let mut prev0 = 0;
    let mut prev1 = 1;
    let mut prev2 = 1;
    let mut tn = 0;

    for _ in 3..=n {
        tn = prev0 + prev1 + prev2;
        prev0 = prev1;
        prev1 = prev2;
        prev2 = tn;
    }

    tn
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_1137::tribonacci;

    #[test]
    fn test_tribonacci_1() {
        assert_eq!(4, tribonacci(4));
    }

    #[test]
    fn test_tribonacci_2() {
        assert_eq!(1389537, tribonacci(25));
    }
}
