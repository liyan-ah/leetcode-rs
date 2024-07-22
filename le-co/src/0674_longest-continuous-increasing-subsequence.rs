impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let mut cur_len = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                cur_len += 1;
                continue;
            }
            max_len = if cur_len > max_len { cur_len } else { max_len };
            cur_len = 1;
        }
        max_len = if cur_len > max_len { cur_len } else { max_len };
        max_len
    }
}
