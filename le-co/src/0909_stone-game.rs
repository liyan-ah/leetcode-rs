impl Solution {
    #[allow(dead_code)]
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(piles.len(), vec![]);
        let mut i = 0;
        while i < piles.len() {
            dp[i].resize(piles.len(), 0);
            dp[i][i] = piles[i];
            i += 1;
        }
        i = piles.len() - 2;
        loop {
            let mut j = i + 1;
            loop {
                if j >= piles.len() {
                    break;
                }
                dp[i][j] = std::cmp::max(piles[i] - dp[i + 1][j], piles[j] - dp[i][j - 1]);
                j += 1;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        if dp[0][piles.len() - 1] > 0 {
            true
        } else {
            false
        }
    }
}
