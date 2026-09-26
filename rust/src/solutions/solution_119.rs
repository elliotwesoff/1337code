fn get_row(row_index: i32) -> Vec<i32> {
    match row_index {
        0 => return vec![1],
        1 => return vec![1, 1],
        _ => (),
    }

    let mut row = Vec::with_capacity(row_index as usize + 1);

    row.push(1);
    row.push(1);

    for i in 2..=row_index {
        let mut prev = 1;

        for j in 1..i {
            let cur = row[j as usize];
            row[j as usize] = prev + cur;
            prev = cur;
        }

        row.push(1);
    }

    row
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_119::get_row;

    #[test]
    fn test_get_row_1() {
        assert_eq!([1], *get_row(0));
    }

    #[test]
    fn test_get_row_2() {
        assert_eq!([1, 1], *get_row(1));
    }

    #[test]
    fn test_get_row_3() {
        assert_eq!([1, 3, 3, 1], *get_row(3));
    }

    #[test]
    fn test_get_row_4() {
        assert_eq!([1, 4, 6, 4, 1], *get_row(4));
    }
}
