impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s_bytes = s.as_bytes();
        if s.len() <= 2 {return true;}
        let (mut left, mut right) = (0 as usize, s.len()-1);
        fn palindrome(s_bytes: &[u8], l: usize, r: usize) ->bool{
            let (mut left, mut right) = (l, r);
            while left < right{
                if s_bytes[left] == s_bytes[right]{
                    left += 1;
                    right -= 1;
                    continue;
                }
                return false;
            }
            return true;
        }
        while left < right {
            if s_bytes[left] == s_bytes[right]{
                left += 1;
                right -= 1;
                continue
            }
            // check gap is left
            let pali = palindrome(s_bytes, left+1, right);
            if pali{
                return true;
            }
            let pali = palindrome(s_bytes, left, right-1);
            if pali{
                return true;
            }
            return false;
        }
        return true
    }
}
