impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        use std::cmp::{max, min};
        let mut dp = vec![];
        let len = matrix.len();
        if len == 0 {
            return 0;
        }
        let mut i = 0;
        let mut max_s = 0;
        'init: loop {
            if i >= len {
                break 'init;
            }
            dp.push(vec![0; matrix[i].len()]);
            if matrix[i][0] == '1' {
                dp[i][0] = 1;
                max_s = 1;
            }
            i += 1;
        }
        i = 0;
        'init: loop {
            if i >= matrix[0].len() {
                break 'init;
            }
            if matrix[0][i] == '1' {
                dp[0][i] = 1;
                max_s = 1;
            }
            i += 1;
        }

        i = 1;
        'i: loop {
            if i >= matrix.len() {
                break 'i;
            }
            let mut j = 1;

            'j: loop {
                if j >= matrix[i].len() {
                    break 'j;
                }
                if matrix[i][j] == '1' {
                    dp[i][j] = min(min(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1]) + 1;
                }
                max_s = max(max_s, dp[i][j]);
                j += 1;
            }
            i += 1;
        }

        max_s * max_s
    }
}
