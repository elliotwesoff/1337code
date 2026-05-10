fn length_of_last_word(s: String) -> i32 {
    let iter = s.split_whitespace();

    if let Some(word) = iter.last() {
        word.len() as i32
    } else {
        panic!("you told me there would be at least one word! i have trust issues, you know.");
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_58::length_of_last_word;

    #[test]
    fn test_length_of_last_word_1() {
        let input = String::from("Hello World");
        let output: i32 = 5;
        assert_eq!(output, length_of_last_word(input));
    }

    #[test]
    fn test_length_of_last_word_2() {
        let input = String::from("   fly me   to   the moon  ");
        let output: i32 = 4;
        assert_eq!(output, length_of_last_word(input));
    }

    #[test]
    fn test_length_of_last_word_3() {
        let input = String::from("luffy is still joyboy");
        let output: i32 = 6;
        assert_eq!(output, length_of_last_word(input));
    }

}
