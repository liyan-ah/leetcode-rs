impl Solution {
    #[allow(dead_code)]
    pub fn first_palindrome(words: Vec<String>) -> String {
        fn is_palindrome(word: &String) -> bool {
            let w_b = word.as_bytes();
            let (mut start, mut end) = (0 as usize, w_b.len() - 1);
            while start < end {
                if w_b[start] != w_b[end] {
                    return false;
                }
                start += 1;
                end -= 1;
            }
            true
        }
        let mut pos = 0 as usize;
        while pos < words.len() {
            if is_palindrome(&words[pos]) {
                return words[pos].clone();
            }
            pos += 1;
        }
        String::from("")
    }
}
