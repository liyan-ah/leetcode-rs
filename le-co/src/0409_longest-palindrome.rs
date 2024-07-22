use std::collections::HashMap;
impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome(s: String) -> i32 {
        let mut b_c: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            let count = b_c.entry(c).or_insert(0);
            *count += 1;
        }
        let mut has_single = false;
        let mut length = 0;
        for ele in b_c {
            if ele.1 % 2 == 1 {
                has_single = true;
                length += ele.1 - 1;
            } else {
                length += ele.1;
            }
        }
        if has_single {
            length += 1;
        }

        length
    }
}
