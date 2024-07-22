pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut s, mut e) = (0, nums.len() - 1);
        while s < e {
            if nums[s] >= target {
                return s as i32;
            }
            if nums[e] <= target {
                return (e + 1) as i32;
            }
            let next = (s + e) / 2;
            if nums[next] > target {
                e = next - 1;
            } else {
                s = next + 1;
            }
        }

        (s + 1) as i32
    }
}
