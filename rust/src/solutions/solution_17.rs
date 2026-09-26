pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut nums_map = std::collections::HashMap::new();
    let mut v = vec![];
    let mut s = String::new();

    nums_map.insert('2', "abc");
    nums_map.insert('3', "def");
    nums_map.insert('4', "ghi");
    nums_map.insert('5', "jkl");
    nums_map.insert('6', "mno");
    nums_map.insert('7', "pqrs");
    nums_map.insert('8', "tuv");
    nums_map.insert('9', "wxyz");

    let digits_arr: Vec<String> = digits
        .chars()
        .map(|c| String::from(*nums_map.get(&c).unwrap()))
        .into_iter()
        .collect();

    build_combinations(&mut v, &digits_arr, digits_arr.len(), &mut s, 0);

    v
}

fn build_combinations(
    v: &mut Vec<String>,
    digits_arr: &Vec<String>,
    digits_arr_len: usize,
    s: &mut String,
    i: usize,
) {
    if i >= digits_arr_len {
        v.push(s.clone());
        return;
    }

    for c in digits_arr[i].chars() {
        s.push(c);
        build_combinations(v, digits_arr, digits_arr_len, s, i + 1);
        s.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::solution_17::letter_combinations;

    #[test]
    fn test_letter_combinations_1() {
        let expected = vec![
            String::from("ad"),
            String::from("ae"),
            String::from("af"),
            String::from("bd"),
            String::from("be"),
            String::from("bf"),
            String::from("cd"),
            String::from("ce"),
            String::from("cf"),
        ];
        assert_eq!(expected, letter_combinations(String::from("23")));
    }

    #[test]
    fn test_letter_combinations_2() {
        let expected = vec![String::from("a"), String::from("b"), String::from("c")];
        assert_eq!(expected, letter_combinations(String::from("2")));
    }

    #[test]
    fn test_letter_combinations_3() {
        let expected = vec![
            String::from("gjm"),
            String::from("gjn"),
            String::from("gjo"),
            String::from("gkm"),
            String::from("gkn"),
            String::from("gko"),
            String::from("glm"),
            String::from("gln"),
            String::from("glo"),
            String::from("hjm"),
            String::from("hjn"),
            String::from("hjo"),
            String::from("hkm"),
            String::from("hkn"),
            String::from("hko"),
            String::from("hlm"),
            String::from("hln"),
            String::from("hlo"),
            String::from("ijm"),
            String::from("ijn"),
            String::from("ijo"),
            String::from("ikm"),
            String::from("ikn"),
            String::from("iko"),
            String::from("ilm"),
            String::from("iln"),
            String::from("ilo"),
        ];
        assert_eq!(expected, letter_combinations(String::from("456")));
    }

    #[test]
    fn test_letter_combinations_4() {
        let expected = vec![
            String::from("gjmp"),
            String::from("gjmq"),
            String::from("gjmr"),
            String::from("gjms"),
            String::from("gjnp"),
            String::from("gjnq"),
            String::from("gjnr"),
            String::from("gjns"),
            String::from("gjop"),
            String::from("gjoq"),
            String::from("gjor"),
            String::from("gjos"),
            String::from("gkmp"),
            String::from("gkmq"),
            String::from("gkmr"),
            String::from("gkms"),
            String::from("gknp"),
            String::from("gknq"),
            String::from("gknr"),
            String::from("gkns"),
            String::from("gkop"),
            String::from("gkoq"),
            String::from("gkor"),
            String::from("gkos"),
            String::from("glmp"),
            String::from("glmq"),
            String::from("glmr"),
            String::from("glms"),
            String::from("glnp"),
            String::from("glnq"),
            String::from("glnr"),
            String::from("glns"),
            String::from("glop"),
            String::from("gloq"),
            String::from("glor"),
            String::from("glos"),
            String::from("hjmp"),
            String::from("hjmq"),
            String::from("hjmr"),
            String::from("hjms"),
            String::from("hjnp"),
            String::from("hjnq"),
            String::from("hjnr"),
            String::from("hjns"),
            String::from("hjop"),
            String::from("hjoq"),
            String::from("hjor"),
            String::from("hjos"),
            String::from("hkmp"),
            String::from("hkmq"),
            String::from("hkmr"),
            String::from("hkms"),
            String::from("hknp"),
            String::from("hknq"),
            String::from("hknr"),
            String::from("hkns"),
            String::from("hkop"),
            String::from("hkoq"),
            String::from("hkor"),
            String::from("hkos"),
            String::from("hlmp"),
            String::from("hlmq"),
            String::from("hlmr"),
            String::from("hlms"),
            String::from("hlnp"),
            String::from("hlnq"),
            String::from("hlnr"),
            String::from("hlns"),
            String::from("hlop"),
            String::from("hloq"),
            String::from("hlor"),
            String::from("hlos"),
            String::from("ijmp"),
            String::from("ijmq"),
            String::from("ijmr"),
            String::from("ijms"),
            String::from("ijnp"),
            String::from("ijnq"),
            String::from("ijnr"),
            String::from("ijns"),
            String::from("ijop"),
            String::from("ijoq"),
            String::from("ijor"),
            String::from("ijos"),
            String::from("ikmp"),
            String::from("ikmq"),
            String::from("ikmr"),
            String::from("ikms"),
            String::from("iknp"),
            String::from("iknq"),
            String::from("iknr"),
            String::from("ikns"),
            String::from("ikop"),
            String::from("ikoq"),
            String::from("ikor"),
            String::from("ikos"),
            String::from("ilmp"),
            String::from("ilmq"),
            String::from("ilmr"),
            String::from("ilms"),
            String::from("ilnp"),
            String::from("ilnq"),
            String::from("ilnr"),
            String::from("ilns"),
            String::from("ilop"),
            String::from("iloq"),
            String::from("ilor"),
            String::from("ilos"),
        ];
        assert_eq!(expected, letter_combinations(String::from("4567")));
    }
}
