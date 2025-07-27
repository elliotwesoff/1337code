use std::collections::HashSet;

pub struct Solution217 {
}

impl Solution217 {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hs: HashSet<i32> = HashSet::new();
        nums.into_iter().any(|n| !hs.insert(n))
    }

    pub fn run() {
        println!("*** Running solution 217 ***");

        let input1 = vec![1, 2, 3, 1];
        let r1 = Solution217::contains_duplicate(input1);
        assert_eq!(r1, true);

        let input2 = vec![1, 2, 3, 1];
        let r2 = Solution217::contains_duplicate(input2);
        assert_eq!(r2, true);

        let input3 = vec![1, 2, 3, 1];
        let r3 = Solution217::contains_duplicate(input3);
        assert_eq!(r3, true);

        println!("*** End solution 217 ***");
    }
}
