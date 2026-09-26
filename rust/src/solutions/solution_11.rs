fn max_area(height: Vec<i32>) -> i32 {
    let mut _max_area = 0;
    let mut i = 0;
    let mut j = height.len() - 1;

    while i < j {
        let i_height = height[i];
        let j_height = height[j];
        let current_area = i_height.min(j_height) * (j - i) as i32;

        _max_area = _max_area.max(current_area);

        if i_height < j_height {
            i += 1;
        } else {
            j -= 1;
        }
    }

    _max_area
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_11::max_area;

    #[test]
    fn test_max_area_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(49, max_area(height));
    }

    #[test]
    fn test_max_area_2() {
        let height = vec![1, 1];
        assert_eq!(1, max_area(height));
    }
}
