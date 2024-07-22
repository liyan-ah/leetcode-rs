impl Solution {
    #[allow(dead_code)]
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        // pre: (postive_len,  negetive_len)
        let (mut pre, mut max) = ((0, 0), 0);
        for i in 0..nums.len() {
            max = std::cmp::max(pre.0, max);
            if nums[i] == 0 {
                pre = (0, 0);
                continue;
            }
            if nums[i] > 0 {
                pre.0 += 1;
                pre.1 = if pre.1 > 0 { pre.1 + 1 } else { 0 };
                continue;
            }
            // nums[i] < 0
            let pos = if pre.1 > 0 { pre.1 + 1 } else { 0 };
            let neg = if pre.0 > 0 { pre.0 + 1 } else { 1 };
            pre = (pos, neg);
        }
        std::cmp::max(pre.0, max)
    }
}
