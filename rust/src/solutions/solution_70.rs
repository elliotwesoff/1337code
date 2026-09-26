use std::collections::HashMap;

// recursive solution with memoization
fn climbing_stairs(n: i32) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();

    memo.insert(1, 1);
    memo.insert(2, 2);

    fn _climbing_stairs(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if memo.contains_key(&n) {
            return memo[&n];
        }

        let count = _climbing_stairs(n - 1, memo) + _climbing_stairs(n - 2, memo);

        memo.insert(n, count);

        count
    }

    _climbing_stairs(n, &mut memo)
}

// basically fibonacci sequence, bottom up
fn climbing_stairs2(n: i32) -> i32 {
    let mut prev2 = 1;
    let mut prev = 1;
    let mut count = 0;

    for _ in 2..=n {
        count = prev2 + prev;
        prev2 = prev;
        prev = count;
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_70::{climbing_stairs, climbing_stairs2};

    #[test]
    fn test_climbing_stairs_1() {
        assert_eq!(2, climbing_stairs(2));
    }

    #[test]
    fn test_climbing_stairs_2() {
        assert_eq!(3, climbing_stairs(3));
    }

    #[test]
    fn test_climbing_stairs_3() {
        assert_eq!(1836311903, climbing_stairs(45));
    }

    #[test]
    fn test_climbing_stairs_4() {
        assert_eq!(2, climbing_stairs2(2));
    }

    #[test]
    fn test_climbing_stairs_5() {
        assert_eq!(3, climbing_stairs2(3));
    }

    #[test]
    fn test_climbing_stairs_6() {
        assert_eq!(1836311903, climbing_stairs2(45));
    }
}
