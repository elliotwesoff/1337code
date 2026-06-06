fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, a) in nums.iter().enumerate() {
        for (j, b) in nums.iter().enumerate() {
            if a + b == target && i != j {
                return vec![i.try_into().unwrap(), j.try_into().unwrap()];
            }
        }
    }

    panic!("no two sum found");
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_1::two_sum;

    #[test]
    fn test_two_sum_1() {
        let i1 = vec![2, 7, 11, 15];
        let t1 = 9;
        let r1 = two_sum(i1, t1);
        assert_eq!(vec![0, 1], r1);
    }

    #[test]
    fn test_two_sum_2() {
        let i = vec![3, 2, 4];
        let t = 6;
        let r = two_sum(i, t);
        assert_eq!(vec![1, 2], r);
    }

    #[test]
    fn test_two_sum_3() {
        let i = vec![3, 3];
        let t = 6;
        let r = two_sum(i, t);
        assert_eq!(vec![0, 1], r);
    }
}
