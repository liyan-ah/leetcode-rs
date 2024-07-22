impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut s = std::collections::HashSet::new();
        let mut last = None;
        loop {
            let mut round = None;
            for v in nums.iter() {
                let v = *v;
                if s.get(&v).is_some() {
                    continue;
                }
                match round {
                    Some(v1) => {
                        if v > v1 {
                            round = Some(v);
                        }
                    }
                    None => {
                        round = Some(v);
                    }
                }
            }
            if round.is_none() {
                return last.unwrap();
            } else if last.is_none() {
                last = round.clone();
            }
            if s.len() == 2 {
                return round.unwrap();
            }
            s.insert(round.unwrap());
        }
    }
}
