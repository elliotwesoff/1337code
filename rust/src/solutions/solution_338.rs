fn count_bits(n: i32) -> Vec<i32> {
    let mut ans = Vec::with_capacity(n as usize + 1);

    for i in 0..=n {
        let mut x = i;
        let mut count = 0;

        while x != 0 {
            x &= x - 1;
            count += 1;
        }

        ans.push(count);
    }

    ans
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_338::count_bits;

    #[test]
    fn test_count_bits_1() {
        assert_eq!([0], *count_bits(0));
    }

    #[test]
    fn test_count_bits_2() {
        assert_eq!([0,1], *count_bits(1));
    }

    #[test]
    fn test_count_bits_3() {
        assert_eq!([0,1,1], *count_bits(2));
    }

    #[test]
    fn test_count_bits_4() {
        assert_eq!([0,1,1,2,1,2], *count_bits(5));
    }

    #[test]
    fn test_count_bits_5() {
        assert_eq!([0,1,1,2,1,2,2,3,1,2,2,3,2,3,3,4,1], *count_bits(16));
    }
}
