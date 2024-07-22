impl Solution {
    #[allow(dead_code)]
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let (mut up, mut down) = (1, 1);
        nums.windows(2).for_each(|x| {
            if x[0] > x[1] {
                up = down + 1;
            }
            if x[0] < x[1] {
                down = up + 1;
            }
        });
        std::cmp::max(up, down)
    }
}
