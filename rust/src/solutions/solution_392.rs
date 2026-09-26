fn is_subsequence(s: String, t: String) -> bool {
    let mut s_chars = s.chars().peekable();
    let mut t_chars = t.chars().peekable();

    loop {
        match s_chars.peek() {
            Some(&sc) => match t_chars.peek() {
                Some(&tc) => {
                    if sc == tc {
                        s_chars.next();
                    }

                    t_chars.next();
                }
                None => return false,
            },
            None => return true,
        }
    }
}

// AI refactor
fn is_subsequence_ai(s: String, t: String) -> bool {
    let mut t_chars = t.chars();
    s.chars()
        .all(|s_char| t_chars.any(|t_char| t_char == s_char))
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_392::is_subsequence;

    #[test]
    fn test_is_subsequence_1() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        assert_eq!(true, is_subsequence(s, t));
    }

    #[test]
    fn test_is_subsequence_2() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");
        assert_eq!(false, is_subsequence(s, t));
    }

    #[test]
    fn test_is_subsequence_3() {
        let s = String::from("b");
        let t = String::from("c");
        assert_eq!(false, is_subsequence(s, t));
    }
}
