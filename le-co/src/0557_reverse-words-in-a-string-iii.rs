
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut bytes = s.into_bytes();
        fn reverse(sub: &mut [u8], begin: usize, end: usize) {
            let mut begin = begin;
            let mut end = end;
            while begin < end {
                let tmp = sub[end];
                sub[end] = sub[begin];
                sub[begin] = tmp;
                begin += 1;
                end -= 1;
            }
        }
        let (mut begin, mut end) = (0, 0);
        while end < bytes.len() {
            // find space, get one word of bytes[begin, end]
            if bytes[end] == b' ' {
                reverse(&mut bytes, begin, end - 1);
                // move to next words, whitespace cannot be the end of words
                begin = end + 1;
            }
            end += 1;
        }
        if end > begin {
            reverse(&mut bytes, begin, end - 1);
        }
        match std::str::from_utf8(&bytes) {
            Ok(v) => return v.to_string(),
            Err(_e) => return "".to_string(),
        };
    }
}
