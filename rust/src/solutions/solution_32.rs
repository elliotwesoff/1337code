use std::collections::VecDeque;

fn descend(deq: &mut VecDeque<char>) -> i32 {
    match deq.pop_front() {
        Some(c) => match c {
            '(' => {
                let result = descend(deq);
                if result % 2 == 0 { result } else { 0 }
            }
            ')' => 1,
            _ => panic!("huh?"),
        },
        None => 0,
    }
}

fn longest_valid_parenthesis(s: String) -> i32 {
    let mut deq: VecDeque<char> = s.chars().collect();
    let mut count = 0;

    while !deq.is_empty() {
        count += descend(&mut deq);
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_32::longest_valid_parenthesis;

    #[test]
    fn test_longest_valid_parenthesis_1() {
        assert_eq!(2, longest_valid_parenthesis(String::from("(()")));
    }

    #[test]
    fn test_longest_valid_parenthesis_2() {
        assert_eq!(4, longest_valid_parenthesis(String::from(")()())")));
    }

    #[test]
    fn test_longest_valid_parenthesis_3() {
        assert_eq!(0, longest_valid_parenthesis(String::from("")));
    }

    #[test]
    fn test_longest_valid_parenthesis_4() {
        assert_eq!(2, longest_valid_parenthesis(String::from("()(()")));
    }

    #[test]
    fn test_longest_valid_parenthesis_5() {
        assert_eq!(2, longest_valid_parenthesis(String::from("((((()")));
    }

    #[test]
    fn test_longest_valid_parenthesis_6() {
        assert_eq!(2, longest_valid_parenthesis(String::from("()))))")));
    }

    #[test]
    fn test_longest_valid_parenthesis_7() {
        assert_eq!(6, longest_valid_parenthesis(String::from("()(())")));
    }
}
