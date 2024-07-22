impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for chunk in nums.chunks(2) {
            let [freq, val] = chunk else { todo!() };
            res.append(&mut vec![*val; freq.clone() as usize]);
        }
        res
    }
}
