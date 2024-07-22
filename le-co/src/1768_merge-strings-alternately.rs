impl Solution {
    #[allow(dead_code)]
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let w_b1 = word1.into_bytes();
        let w_b2 = word2.into_bytes();
        let mut new_word: Vec<u8> = vec![];
        let (mut p1, mut p2) = (0 as usize, 0 as usize);
        while p1 < w_b1.len() && p2 < w_b2.len() {
            new_word.push(w_b1[p1]);
            new_word.push(w_b2[p2]);
            p1 += 1;
            p2 += 1;
        }
        while p1 < w_b1.len() && p2 >= w_b2.len() {
            new_word.push(w_b1[p1]);
            p1 += 1;
        }
        while p1 >= w_b1.len() && p2 < w_b2.len() {
            new_word.push(w_b2[p2]);
            p2 += 1;
        }

        String::from_utf8(new_word).unwrap()
    }
}
