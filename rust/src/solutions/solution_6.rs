use std::num;

fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 || num_rows as usize >= s.len() {
        return s;
    }

    let mut i = 0;
    let mut step = 1;
    let mut strings =
        vec![String::with_capacity((s.len() / num_rows as usize) + 1); num_rows as usize];

    for c in s.chars() {
        strings[i].push(c);

        if i == 0 {
            step = 1;
        } else if i as i32 == num_rows - 1 {
            step = -1;
        }

        i = (i as i32 + step) as usize;
    }

    strings.join("")
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_6::convert;

    #[test]
    fn test_convert_1() {
        assert_eq!("PAHNAPLSIIGYIR", convert(String::from("PAYPALISHIRING"), 3));
    }

    #[test]
    fn test_convert_2() {
        assert_eq!("PINALSIGYAHRPI", convert(String::from("PAYPALISHIRING"), 4));
    }

    #[test]
    fn test_convert_3() {
        assert_eq!("A", convert(String::from("A"), 1));
    }

    #[test]
    fn test_convert_4() {
        assert_eq!("AB", convert(String::from("AB"), 1));
    }
}
