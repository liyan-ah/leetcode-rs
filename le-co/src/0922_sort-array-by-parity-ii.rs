impl Solution {
    #[allow(dead_code)]
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let (mut ab_odd, mut ab_even) = (1 as usize, 0 as usize);
        while ab_odd < nums.len() && ab_even < nums.len() {
            // find next ab_odd
            while ab_odd < nums.len() {
                if nums[ab_odd] % 2 == 0 {
                    break;
                }
                ab_odd += 2;
            }
            // find next ab_even
            while ab_even < nums.len() {
                if nums[ab_even] % 2 == 1 {
                    break;
                }
                ab_even += 2;
            }
            if ab_odd > nums.len() || ab_even > nums.len() {
                break;
            }
            if nums[ab_odd] % 2 == 1 || nums[ab_even] % 2 == 0 {
                break;
            }
            // swap abnormal
            nums.swap(ab_odd, ab_even);
        }
        nums
    }
}
