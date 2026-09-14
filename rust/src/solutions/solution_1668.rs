fn max_repeating(sequence: String, word: String) -> i32 {
    let seq_len = sequence.len() as i32;
    let word_len = word.len() as i32;
    let word_str = &word[..];
    let mut i: i32 = 0;
    let mut k_match_start: i32 = -1;
    let mut count = 0;
    let mut current_count = 0;

    if seq_len == 1 {
        return (sequence == word) as i32;
    }

    while i + word_len <= seq_len {
        let subseq = &sequence[(i as usize)..(i as usize + word_len as usize)];

        if subseq == word_str {
            current_count += 1;

            if current_count > count {
                count = current_count;
            }

            if k_match_start == -1 {
                k_match_start = i;
            }

            i += word_len;
        } else {
            if k_match_start != -1 {
                i = k_match_start + 1;
                k_match_start = -1;
            } else {
                i += 1;
            }

            current_count = 0;
        }
    }

    count
}

// AI refactor is really good... v.v
// pub fn max_repeating(sequence: String, word: String) -> i32 {
//     let mut k = 0;
//     let mut repeated_word = word.clone();
//
//     // Keep incrementing k as long as sequence contains the repeated word
//     while sequence.contains(&repeated_word) {
//         k += 1;
//         repeated_word.push_str(&word);
//     }
//
//     k
// }

// AI DP solution (mine is sliding window with backtracking)
// pub fn max_repeating(sequence: String, word: String) -> i32 {
//     let seq_len = sequence.len();
//     let word_len = word.len();
//
//     if seq_len < word_len {
//         return 0;
//     }
//
//     // dp[i] stores the max repeating count of 'word' ending at index i
//     let mut dp = vec![0; seq_len + 1];
//     let mut max_k = 0;
//
//     // Iterate through the sequence to find matches
//     for i in word_len..=seq_len {
//         // Check if the substring ending at i matches 'word'
//         if &sequence[i - word_len..i] == word {
//             // DP Transition: current count = 1 + count from where this match started
//             dp[i] = dp[i - word_len] + 1;
//             max_k = max_k.max(dp[i]);
//         }
//     }
//
//     max_k
// }

#[cfg(test)]
mod tests {
    use crate::solutions::solution_1668::max_repeating;

    #[test]
    fn test_max_repeating_1() {
        assert_eq!(2, max_repeating(String::from("ababc"), String::from("ab")));
    }

    #[test]
    fn test_max_repeating_2() {
        assert_eq!(1, max_repeating(String::from("ababc"), String::from("ba")));
    }

    #[test]
    fn test_max_repeating_3() {
        assert_eq!(0, max_repeating(String::from("ababc"), String::from("ac")));
    }

    #[test]
    fn test_max_repeating_5() {
        assert_eq!(3, max_repeating(String::from("aaa"), String::from("a")));
    }

    #[test]
    fn test_max_repeating_6() {
        assert_eq!(5, max_repeating(String::from("aaabaaaabaaabaaaabaaaabaaaabaaaaba"), String::from("aaaba")));
    }

    #[test]
    fn test_max_repeating_7() {
        assert_eq!(0, max_repeating(String::from("a"), String::from("b")));
    }

}
