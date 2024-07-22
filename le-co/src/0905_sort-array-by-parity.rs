impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut top_even = 0 as usize;
        let mut pos = 0 as usize;
        while pos < nums.len() {
            if nums[pos] % 2 == 0 {
                nums.swap(top_even, pos);
                top_even += 1;
            }
            pos += 1;
        }

        nums
    }
}
