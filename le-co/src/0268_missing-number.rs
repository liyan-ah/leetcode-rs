impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let l = nums.len() as i32;
        let mut sum = ((1 + l) * l) / 2;
        for v in nums.iter() {
            sum -= *v;
        }
        sum
    }
}
