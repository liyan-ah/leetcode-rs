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
