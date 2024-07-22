impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut str = word1.concat();
        let mut i = 0;
        while i < word2.len() {
            let w = &word2[i];
            str = str.replacen(w, "", 1);
            if str.len() == 0 && i < word2.len() - 1 {
                return false;
            }
            i += 1;
        }
        if str.len() != 0 {
            return false;
        }

        true
    }
}
