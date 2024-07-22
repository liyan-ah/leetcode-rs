impl Solution {
    #[allow(dead_code)]
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut w_b = word.into_bytes();
        let (mut start, mut end) = (0 as usize, 0 as usize);
        while start < w_b.len() {
            if char::from(w_b[start]) == ch {
                end = start;
                break;
            }
            start += 1;
        }
        start = 0 as usize;
        while start < end {
            w_b.swap(start, end);
            start += 1;
            end -= 1;
        }

        String::from_utf8(w_b).unwrap()
    }
}
