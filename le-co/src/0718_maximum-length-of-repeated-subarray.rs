impl Solution {
    #[allow(dead_code)]
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(nums1.len() + 1, vec![]);
        for i in 0..nums1.len() + 1 {
            dp[i].resize(nums2.len() + 1, 0);
        }
        if nums1.len() == 1 || nums2.len() == 1 {
            return if nums1[nums1.len() - 1] == nums2[nums2.len() - 1] {
                1
            } else {
                0
            };
        }
        let mut i = nums1.len() - 1;
        let mut max = 0;
        'nums1: loop {
            let mut j = nums2.len() - 1;
            'nums2: loop {
                if nums1[i] == nums2[j] {
                    dp[i][j] = dp[i + 1][j + 1] + 1;
                    max = std::cmp::max(dp[i][j], max);
                } else {
                    dp[i][j] = 0;
                }
                if j == 0 {
                    break 'nums2;
                }
                j -= 1;
            }
            if i == 0 {
                break 'nums1;
            }
            i -= 1;
        }
        //for i in 0..dp.len() {
        //    println!("{:?}", dp[i]);
        //}
        max
    }
}
