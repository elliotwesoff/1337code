fn str_str(haystack: String, needle: String) -> i32 {
    let haystack_len = haystack.len();
    let needle_len = needle.len();
    let mut i: usize = 0;

    if haystack_len == needle_len {
        return match haystack == needle {
            true => 0,
            false => -1
        };
    }

    while i < haystack_len {
        let j = std::cmp::min(i + needle_len, haystack_len);

        let hay = if i == j {
            &haystack[haystack_len - 1..]
        } else {
            &haystack[i..j]
        };

        if hay == needle {
            return i.try_into().unwrap_or(-1);
        }

        i += 1;
    }

    -1
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_28::str_str;

    #[test]
    fn test_str_str_1() {
        assert_eq!(
            str_str("sadbutsad".to_string(), "sad".to_string()),
            0,
            "oops!"
        );
    }
    #[test]
    fn test_str_str_2() {
        assert_eq!(
            str_str("leetcode".to_string(), "leeto".to_string()),
            -1,
            "yikes!"
        );
    }
    #[test]
    fn test_str_str_3() {
        assert_eq!(
            str_str("a".to_string(), "a".to_string()),
            0,
            "uh oh!"
        );
    }
    #[test]
    fn test_str_str_4() {
        assert_eq!(
            str_str("abc".to_string(), "c".to_string()),
            2,
            "seriously..."
        );
    }
    #[test]
    fn test_str_str_5() {
        assert_eq!(
            str_str("mississippi".to_string(), "pi".to_string()),
            9,
            "kms..."
        );
    }
}