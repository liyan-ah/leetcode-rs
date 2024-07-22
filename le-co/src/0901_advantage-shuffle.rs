impl Solution {
    #[allow(dead_code)]
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums_1 = nums1.to_vec();
        let mut nums_2 = nums2.to_vec();
        nums_1.sort();
        nums_2.sort();

        let mut remain = Vec::new();
        let mut hash_table = std::collections::HashMap::new();

        let (mut i, mut j) = (0, 0);
        while i < nums_1.len() {
            if nums_1[i] > nums_2[j] {
                hash_table
                    .entry(nums_2[j])
                    .or_insert(Vec::new())
                    .push(nums_1[i]);
                j += 1;
            } else {
                // not satisfied
                remain.push(nums_1[i]);
            }
            i += 1;
        }

        let mut ans = Vec::new();
        for pos in 0..nums2.len() {
            if let Some(val) = hash_table.get_mut(&nums2[pos]) {
                ans.push(val.remove(0));
                if val.is_empty() {
                    hash_table.remove(&nums2[pos]);
                }
            } else {
                ans.push(remain.pop().unwrap());
            }
        }

        ans
    }
}
