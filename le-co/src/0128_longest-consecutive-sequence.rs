impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut dp: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut longest = 0;
        let mut i = 0;
        loop {
            if i >= nums.len() {
                break;
            }
            let iter = nums[i];
            if dp.get(&iter).is_some() {
                i += 1;
                continue;
            }
            let l;
            if let Some(v) = dp.get(&(iter - 1)) {
                l = v.clone();
            } else {
                l = 0;
            }
            let r;
            if let Some(v) = dp.get(&(iter + 1)) {
                r = v.clone();
            } else {
                r = 0;
            }
            let mut dv = l + r + 1;
            if let Some(v) = dp.get(&iter) {
                if *v > dv {
                    dv = v.clone();
                }
            }
            dp.insert(iter, dv);
            dp.insert(iter - l, dv);
            dp.insert(iter + r, dv);
            i += 1;
            if longest < dv {
                longest = dv;
            }
        }
        longest
    }
}
