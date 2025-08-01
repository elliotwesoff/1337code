pub struct Solution27 { }

impl Solution27 {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|n| *n != val);
        nums.len().try_into().unwrap()
    }

    pub fn run() {
        println!("*** Running solution 27 ***");

        let mut nums1 = vec![3,2,2,3];
        let val1 = 3;
        let r1 = Solution27::remove_element(&mut nums1, val1);
        assert_eq!(r1, 2);

        println!("*** End solution 27 ***");
    }
}
