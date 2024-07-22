impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut letters = s.chars().filter(|x| x.is_alphanumeric());
        while let(Some(c1), Some(c2)) = (letters.next(), letters.next_back()){
            if !(c1.eq_ignore_ascii_case(&c2)){
                return false;
            }
        }
        true
    }
}

