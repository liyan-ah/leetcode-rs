impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let [.., x1, x2, x3] = nums[..] else { todo!() };
        if nums.len() == 3 {
            return x1 * x2 * x3;
        }
        if nums.len() > 3 {
            if nums[0] * nums[1] * x3 > x1 * x2 * x3 {
                return nums[0] * nums[1] * x3;
            }
        }

        x1 * x2 * x3
    }
}
