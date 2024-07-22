impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut fmt = vec![];
        let mut cnt = 0;
        for v in s.chars().into_iter().rev() {
            if !v.is_ascii_alphanumeric() {
                continue;
            }
            if cnt % k == 0 && cnt != 0 {
                fmt.push('-');
            }
            if v.is_ascii_alphabetic() {
                fmt.push(v.to_ascii_uppercase());
            } else {
                fmt.push(v);
            }
            cnt += 1;
        }

        fmt.into_iter().rev().collect()
    }
}
