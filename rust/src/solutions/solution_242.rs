use std::collections::HashMap;

fn is_anagram(s: String, t: String) -> bool {
    let mut s_map: HashMap<char, i32> = HashMap::new();
    let mut t_map: HashMap<char, i32> = HashMap::new();

    s.chars().for_each(|c| {
        *s_map.entry(c).or_insert(1) += 1;
    });

    t.chars().for_each(|c| {
        *t_map.entry(c).or_insert(1) += 1;
    });

    for (c, &s_count) in s_map.iter() {
        match t_map.get(&c) {
            Some(&t_count) => {
                if s_count != t_count {
                    return false;
                }
            },
            None => {
                return false;
            }
        }
    }

    for (c, &t_count) in t_map.iter() {
        match s_map.get(&c) {
            Some(&s_count) => {
                if s_count != t_count {
                    return false;
                }
            },
            None => {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_242::is_anagram;

    #[test]
    fn test_is_anagram_1() {
        let s1_1 = String::from("anagram");
        let s1_2 = String::from("nagaram");
        let r1 = is_anagram(s1_1, s1_2);
        assert_eq!(r1, true);
    }
    #[test]
    fn test_is_anagram_2() {
        let s2_1 = String::from("dog");
        let s2_2 = String::from("god");
        let r2 = is_anagram(s2_1, s2_2);
        assert_eq!(r2, true);
    }
    #[test]
    fn test_is_anagram_3() {
        let s3_1 = String::from("a");
        let s3_2 = String::from("ab");
        let r3 = is_anagram(s3_1, s3_2);
        assert_eq!(r3, false);
    }
}