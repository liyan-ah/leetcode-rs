impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut idx: std::collections::HashMap<i32, std::collections::HashSet<_>> =
            std::collections::HashMap::new();
        let mut i = 0;
        let mut count = 0;
        while i < nums.len() {
            match idx.get_mut(&nums[i]) {
                Some(v) => {
                    count += v.len() as i32;
                    v.insert(i);
                }
                None => {
                    let mut v = std::collections::HashSet::new();
                    v.insert(i);
                    idx.insert(nums[i], v);
                }
            }
            i += 1;
        }
        count
    }
}
