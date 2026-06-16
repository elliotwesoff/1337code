fn divisor_game(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_1025::divisor_game;

    #[test]
    fn test_divisor_game_1() {
        assert_eq!(true, divisor_game(2));
    }

    #[test]
    fn test_divisor_game_2() {
        assert_eq!(false, divisor_game(3));
    }
}
