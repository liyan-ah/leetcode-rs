impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut left = 0;
        let mut right = s.len() - 1;
        let s_c: Vec<char> = s.chars().collect();
        while left < right {
            if s_c[left] != s_c[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
