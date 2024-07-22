impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let mut score: Vec<i32> = vec![0];
        let mut i = 0;
        while i < s_bytes.len() {
            if s_bytes[i] == ('(' as u8) {
                score.push(0);
            } else {
                let mut cur = score.pop().unwrap();
                let mut last = score.pop().unwrap();
                if cur == 0 {
                    cur = 1;
                } else {
                    cur *= 2;
                }
                last += cur;
                score.push(last);
            }
            i += 1;
        }
        return score[0];
    }
}
