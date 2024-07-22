impl Solution {
    #[allow(dead_code)]
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut new = nums.clone();
        let (mut pos, mut neg) = (0 as usize, 1 as usize);
        let mut p = 0 as usize;
        while p < nums.len() {
            if nums[p] < 0 {
                new[neg] = nums[p];
                neg += 2;
            } else {
                new[pos] = nums[p];
                pos += 2;
            }
            p += 1;
        }
        new
    }
}
