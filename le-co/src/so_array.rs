#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
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

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut arr_clone = arr.clone();
        arr_clone.sort();
        let mut rank = HashMap::new();
        let mut rank_v = 1;
        for v in arr_clone.iter() {
            if let None = rank.get(&v) {
                rank.insert(v, rank_v);
                rank_v += 1;
            }
        }

        let mut rank_r = vec![];
        for v in arr.iter() {
            let rank_v = rank.get(&v).expect("elem should in");
            rank_r.push(*rank_v);
        }

        rank_r
    }
}
