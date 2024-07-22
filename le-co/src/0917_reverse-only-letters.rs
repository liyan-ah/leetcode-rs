impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut s_b = s.into_bytes();
        let (mut left, mut right) = (0 as usize, s_b.len() - 1);
        while left < right {
            // find left
            while left < right {
                if s_b[left].is_ascii_alphabetic() {
                    break;
                }
                left += 1;
            }
            while right > left {
                if s_b[right].is_ascii_alphabetic() {
                    break;
                }
                right -= 1;
            }
            // usize 0-1 would overflow
            if right == 0 {
                break;
            }
            s_b.swap(left, right);
            left += 1;
            right -= 1;
        }
        std::str::from_utf8(&s_b).unwrap().to_string()
    }
}
