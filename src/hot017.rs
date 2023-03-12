struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let D_TO_C = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let mut res = vec!["".to_string()];
        for digit in digits.bytes().map(|x| (x - b'0') as usize - 2) {
            let mut temp = vec![];
            for s in res {
                for c in &D_TO_C[digit] {
                    temp.push(format!("{}{}", &s, c));
                }
            }
            res = temp;
        }
        res
    }
}
