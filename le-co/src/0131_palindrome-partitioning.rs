impl Solution {
    #[allow(dead_code)]
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = vec![];
        let s_b = s.as_bytes();
        let mut dp: Vec<Vec<bool>> = vec![];
        dp.resize(s_b.len(), vec![]);
        let mut i = s_b.len() - 1;
        loop {
            dp[i].resize(s_b.len(), true);
            let mut j = i + 1;
            while j < s_b.len() {
                if i >= j {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = if dp[i + 1][j - 1] && s_b[i] == s_b[j] {
                        true
                    } else {
                        false
                    };
                }
                j += 1;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        let mut partition: Vec<String> = vec![];
        self::Solution::dfs(&mut res, &mut partition, &dp, 0, &s);

        res
    }

    #[allow(dead_code)]
    fn dfs(
        res: &mut Vec<Vec<String>>,
        partition: &mut Vec<String>,
        dp: &Vec<Vec<bool>>,
        pos: usize,
        s: &String,
    ) {
        if pos == s.len() {
            let round = partition.to_vec();
            res.push(round);
            return;
        }
        for i in pos..s.len() {
            if dp[pos][i] {
                partition.push(s[pos..i + 1].to_string());
                self::Solution::dfs(res, partition, dp, i + 1, s);
                partition.pop();
            }
        }
    }
}
