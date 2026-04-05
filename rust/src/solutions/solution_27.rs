fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|n| *n != val);
    nums.len().try_into().unwrap()
}


#[cfg(test)]
mod tests {
    use crate::solutions::solution_27::remove_element;

    #[test]
    fn test_remove_element_1() {
        let mut nums1 = vec![3,2,2,3];
        let val1 = 3;
        let r1 = remove_element(&mut nums1, val1);
        assert_eq!(r1, 2);
        
    }
}
