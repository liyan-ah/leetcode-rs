impl Solution {
    #[allow(dead_code)]
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        // pre: no delete, delete one
        let (mut pre, mut max) = ((arr[0], 0), arr[0]);
        for i in 1..arr.len() {
            // if delete this
            pre.1 = std::cmp::max(pre.0, pre.1 + arr[i]);
            pre.0 = std::cmp::max(pre.0 + arr[i], arr[i]);
            max = std::cmp::max(max, pre.0);
            max = std::cmp::max(max, pre.1);
        }
        max
    }
}
