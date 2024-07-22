impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let (mut s, mut l, mut i) = (-1, -1, 0);
        for idx in 0..nums.len() {
            if l < nums[idx] {
                i = idx;
                s = l;
                l = nums[idx];
                continue;
            }
            if s < nums[idx] {
                s = nums[idx];
            }
        }
        return if l >= 2 * s { i as i32 } else { -1 };
    }
}
