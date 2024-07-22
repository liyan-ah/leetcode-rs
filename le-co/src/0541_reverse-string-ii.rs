
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut s_b = s.into_bytes();
        if s_b.len() <= 1 {
            return std::str::from_utf8(&s_b).unwrap().to_string();
        }
        let round = s_b.len() / (2 * k) as usize;
        let mut i = 0 as usize;
        while i < round {
            // in these round, first k [i*2k, i*2k+k),
            let mut left = 2 * i * k;
            let mut right = 2 * i * k + k - 1;
            while left < right {
                s_b.swap(left, right);
                left += 1;
                right -= 1;
            }
            i += 1;
        }
        let rest = s_b.len() - 2 * k * i;
        if rest > k {
            // [2*k*i, 2*k*i+k) reverse
            let mut left = 2 * k * i;
            let mut right = 2 * k * i + k - 1;
            while left < right {
                s_b.swap(left, right);
                left += 1;
                right -= 1;
            }
        } else {
            // [2*k*i, len) reverse
            let mut left = 2 * k * i;
            let mut right = s_b.len() - 1;
            while left < right {
                s_b.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        std::str::from_utf8(&s_b).unwrap().to_string()
    }
}
