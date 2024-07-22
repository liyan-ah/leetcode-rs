impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut m_nums = nums;
        m_nums.sort();
        let mut m_k = k;
        for p in 0..m_nums.len() {
            if m_k <= 0 {
                break;
            }
            // positive and k > 0, try reverse left
            if m_nums[p] >= 0 {
                // rest m_k is even, trans some value even times
                if m_k % 2 == 0 {
                    m_k = 0;
                    break;
                }
                // rest m_k is odd, check abs value
                if p == 0 {
                    m_nums[p] = -1 * m_nums[p];
                    m_k = 0;
                    break;
                }
                if m_nums[p] > m_nums[p - 1] {
                    m_nums[p - 1] = -1 * m_nums[p - 1];
                    m_k = 0;
                    break;
                }
                m_nums[p] = -1 * m_nums[p];
                m_k = 0;
                break;
            }
            // negetive and and k > 0
            if m_k > 0 {
                m_nums[p] = -1 * m_nums[p];
            }
            m_k -= 1;
        }
        if m_k > 0 && m_k % 2 == 1 {
            let p = m_nums.len() - 1;
            m_nums[p] = m_nums[p] * -1;
        }
        let sum = m_nums.iter().fold(0, |acc, x| acc + x);
        sum
    }
}
