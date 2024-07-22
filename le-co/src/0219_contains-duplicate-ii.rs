impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut p_h: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
        for i in 0..nums.len() {
            match p_h.get_mut(&nums[i]) {
                Some(v) => {
                    if (i - *v) <= k.try_into().unwrap() {
                        return true;
                    }
                    p_h.insert(nums[i], i);
                }
                None => {
                    p_h.insert(nums[i], i);
                }
            }
        }
        false
    }
}
