use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hs: HashSet<i32> = HashSet::new();
    nums.into_iter().any(|n| !hs.insert(n))
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_217::contains_duplicate;

    #[test]
    fn test_contains_duplicate_1() {
        let input1 = vec![1, 2, 3, 1];
        let r1 = contains_duplicate(input1);
        assert_eq!(r1, true);
    }
    #[test]
    fn test_contains_duplicate_2() {
        let input2 = vec![1, 2, 3, 1];
        let r2 = contains_duplicate(input2);
        assert_eq!(r2, true);
    }
    #[test]
    fn test_contains_duplicate_3() {
        let input3 = vec![1, 2, 3, 1];
        let r3 = contains_duplicate(input3);
        assert_eq!(r3, true);
    }
}