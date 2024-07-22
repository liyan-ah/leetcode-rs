impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len()).for_each(|i| {
            let idx = (nums[i].abs() - 1) as usize;
            nums[idx] = -nums[idx].abs();
        });
        let mut ret = vec![];
        (0..nums.len()).for_each(|i| {
            if nums[i] > 0 {
                ret.push((i + 1) as i32);
            }
        });
        ret
    }
}
