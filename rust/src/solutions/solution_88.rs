fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = 0;
    let mut j: usize = 0;
    let mut extended_m = m;

    while j < n as usize {
        let n1 = nums1[i];
        let n2 = nums2[j];

        if n2 <= n1 {
            nums1.resize(nums1.len() - 1, 0);
            nums1.insert(i, n2);
            extended_m += 1;
            j += 1;
        } else if i >= extended_m as usize {
            nums1[i] = n2;
            j += 1;
        }

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_88::merge;

    #[test]
    fn test_merge() {
        let mut l1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut l2 = vec![2, 5, 6];
        let n = 3;
        let expected = vec![1, 2, 2, 3, 5, 6];

        merge(&mut l1, m, &mut l2, n);

        assert_eq!(expected, l1);
    }

    #[test]
    fn test_merge_2() {
        let mut l1 = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
        let m = 6;
        let mut l2 = vec![1, 2, 2];
        let n = 3;
        let expected = vec![-1, 0, 0, 1, 2, 2, 3, 3, 3];

        merge(&mut l1, m, &mut l2, n);

        assert_eq!(expected, l1);
    }

    #[test]
    fn test_merge_3() {
        let mut l1 = vec![0];
        let m = 0;
        let mut l2 = vec![1];
        let n = 1;
        let expected = vec![1];

        merge(&mut l1, m, &mut l2, n);

        assert_eq!(expected, l1);
    }
}
