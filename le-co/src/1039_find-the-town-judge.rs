impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 && trust.len() == 0 {
            return 1;
        }
        let mut mark = std::collections::HashMap::with_capacity(n as usize);
        for it in trust.iter() {
            let [ai, bi, ..] = it[..] else { todo!() };
            match mark.get_mut(&bi) {
                Some(v) => {
                    if *v != -1 {
                        *v += 1;
                    }
                }
                None => {
                    mark.insert(bi, 1);
                }
            }
            mark.insert(ai, -1);
        }
        let mut judge = -1;
        for (&k, &v) in mark.iter() {
            if v == n - 1 {
                judge = k;
            }
        }
        judge
    }
}
