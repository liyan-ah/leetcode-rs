use std::iter::FromIterator;
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut last_len: i32 = 0;
        let mut last_char: char = chars[0];
        let mut comp_size: i32 = 0;
        let mut comp_char: Vec<char> = Vec::new();

        fn digit_len(digit: i32) -> (i32, Vec<char>) {
            let mut d_len: i32 = 2;
            let mut digit_ = digit;
            let mut digit_vec: Vec<char> = Vec::new();
            if digit_ == 0 {
                return (0, digit_vec);
            }
            if digit_ == 1 {
                return (1, digit_vec);
            }
            let les_10 = digit_ % 10;
            digit_ = digit_ - les_10;
            while digit_ >= 10 {
                d_len += 1;
                digit_ = digit_ / 10;
            }
            let d_str = digit.to_string();
            digit_vec = Vec::from_iter(d_str.chars());
            return (d_len, digit_vec);
        }

        let mut i: usize = 0;
        while i < chars.len() {
            let char_ = chars[i];
            if char_ != last_char {
                let (digit_len, digit_char) = digit_len(last_len);
                if digit_len > 0 {
                    comp_char.push(last_char);
                    comp_char.extend(digit_char);
                }
                comp_size += digit_len;
                last_char = char_;
                last_len = 1;
                i += 1;
                continue;
            }
            last_len += 1;
            i += 1;
        }
        let (digit_len, digit_char) = digit_len(last_len);
        if digit_len > 0 {
            comp_char.push(last_char);
            comp_char.extend(digit_char);
        }
        comp_size += digit_len;
        let mut i: usize = 0;
        while i < comp_char.len() {
            chars[i] = comp_char[i];
            i += 1;
        }
        return comp_size;
    }
}
