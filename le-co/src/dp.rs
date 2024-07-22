pub struct Solution {}

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

impl Solution {
    #[allow(dead_code)]
    pub fn tribonacci(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![];
        dp.resize((n + 1) as usize, 0);
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 1;
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else if n == 2 {
            return 1;
        }
        let mut i = 0 as usize;
        loop {
            dp[i + 3] = dp[i + 2] + dp[i + 1] + dp[i];
            i += 1;
            if (i + 3) > (n as usize) {
                break;
            }
        }

        dp[n as usize]
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn get_maximum_generated(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![];
        dp.resize((n + 1) as usize, 0);
        dp[0] = 0;
        if n == 0 {
            return dp[0];
        }
        dp[1] = 1;
        if n == 1 {
            return dp[1];
        }
        let mut i = 1;
        let mut max_v = std::cmp::max(dp[0], dp[1]);
        loop {
            if 2 * i + 1 > n || 2 * i > n {
                break;
            }
            dp[(2 * i) as usize] = dp[i as usize];
            dp[(2 * i + 1) as usize] = dp[i as usize] + dp[(i + 1) as usize];
            max_v = std::cmp::max(dp[2 * i as usize], max_v);
            max_v = std::cmp::max(dp[(2 * i + 1) as usize], max_v);
            i += 1;
        }
        max_v
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn fib(n: i32) -> i32 {
        let mut dp = (0 as i32, 1 as i32);
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }

        let mut i = 2;
        let mut value = 0 as i32;
        while i <= n {
            value = dp.0 + dp.1;
            dp.0 = dp.1;
            dp.1 = value;
            i += 1;
        }
        value
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn min_steps(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![];
        dp.resize((n + 1) as usize, 0);
        let mut i = 2 as usize;
        while i <= n as usize {
            dp[i] = -1;
            let mut j = 1 as usize;
            while j * j <= i {
                if i % j == 0 {
                    dp[i] = if dp[i] == -1 {
                        dp[i / j] + j as i32
                    } else {
                        std::cmp::min(dp[i], dp[i / j] + j as i32)
                    };
                    dp[i] = std::cmp::min(dp[i], dp[j] + (i / j) as i32);
                }
                j += 1;
            }
            i += 1;
        }
        dp[n as usize]
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut dp: Vec<i32> = vec![];
        dp.resize((n + 1) as usize, 0);
        dp[0] = 0;
        let mut near_2n = 0 as usize;
        let mut i = 1 as usize;
        while i <= n as usize {
            if i & (i - 1) == 0 {
                near_2n = i;
            }
            dp[i] = dp[i - near_2n] + 1;
            i += 1;
        }
        dp
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn word_break_2(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut dp: Vec<(bool, Vec<String>)> = vec![];
        let s_b = s.as_bytes();
        dp.resize(s_b.len() + 1, (false, vec![]));

        dp[0] = (true, vec![String::from("")]);
        let mut i = 1;
        while i <= s_b.len() {
            let mut j = 0;
            while j < i {
                let sub_str = String::from_utf8(s_b[j..i].to_vec()).unwrap();
                if dp[j].0 == true && word_dict.contains(&sub_str) {
                    dp[i].0 = true;
                    let mut v: Vec<String> = vec![];
                    for iter in &dp[j].1 {
                        let new_arr = iter.to_owned() + &String::from(" ") + &sub_str;
                        v.push(new_arr.trim().to_owned());
                    }
                    dp[i].1.extend(v);
                }
                j += 1;
            }
            i += 1;
        }

        dp[s_b.len()].1.to_owned()
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s_b = s.as_bytes();
        let mut dp: Vec<bool> = vec![];
        dp.resize(s_b.len() + 1, false);
        dp[0] = true;
        let mut i = 1 as usize;
        while i < s_b.len() + 1 {
            let mut j = 0 as usize;
            while j < i {
                let sub_str = String::from_utf8(s_b[j..i].to_vec()).unwrap();
                if word_dict.contains(&sub_str) && dp[j] {
                    dp[i] = true;
                    break;
                }
                j += 1;
            }
            i += 1;
        }
        println!("{:?}", dp);
        dp[s_b.len()]
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let total = nums.iter().fold(0, |s, &a| s + a);
        if target.abs() > total {
            return 0;
        }
        if (total + target) % 2 == 1 {
            return 0;
        }
        let (pos, neg) = ((total + target) / 2, (total - target) / 2);
        let cap = std::cmp::min(pos, neg);

        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(nums.len() + 1, vec![]);
        dp[0].resize((cap + 1) as usize, 0);
        dp[0][0] = 1;
        for i in 1..(nums.len() + 1) as usize {
            dp[i].resize((cap + 1) as usize, 0);
            for j in 0..((cap + 1) as usize) {
                if j < nums[i - 1] as usize {
                    dp[i][j] = dp[i - 1][j];
                    continue;
                }
                dp[i][j] = dp[i - 1][j] + dp[i - 1][(j as i32 - nums[i - 1]) as usize];
            }
        }

        dp[nums.len()][cap as usize]
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(coins.len() + 1, vec![]);
        dp[0].resize((amount + 1) as usize, 0);
        dp[0][0] = 1;
        for i in 1..(coins.len() + 1) as usize {
            dp[i].resize((amount + 1) as usize, 0);
            for j in 0..(amount + 1) as usize {
                if coins[i - 1] > j as i32 {
                    dp[i][j] = dp[i - 1][j];
                    continue;
                }
                dp[i][j] = dp[i - 1][j] + dp[i][j - (coins[i - 1] as usize)];
            }
        }

        dp[coins.len()][amount as usize]
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if coins.len() == 0 || amount == 0 {
            return 0;
        }
        if coins.len() == 1 {
            return if amount % coins[0] == 0 {
                amount / coins[0]
            } else {
                -1
            };
        }
        let mut dp: Vec<i32> = vec![];
        dp.resize((amount + 1) as usize, 0);
        for i in 0..coins.len() {
            if coins[i] < amount {
                dp[(coins[i] as usize)] = 1;
            } else if coins[i] > amount {
                continue;
            } else if coins[i] == amount {
                return 1;
            }
        }
        for i in 1..amount as usize {
            if dp[i] == 0 {
                continue;
            }
            for j in 0..coins.len() {
                let n_p = i + (coins[j]) as usize;
                if n_p > amount as usize {
                    continue;
                }
                if dp[n_p] == 0 {
                    dp[n_p] = dp[i] + 1;
                } else if dp[n_p] != 0 {
                    dp[n_p] = std::cmp::min(dp[i] + 1, dp[n_p]);
                };
            }
        }
        return if dp[amount as usize] == 0 {
            -1
        } else {
            dp[amount as usize]
        };
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn rob_from_to(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        if end - start <= 1 {
            return std::cmp::max(nums[start], nums[end]);
        }
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(nums.len(), vec![]);
        // include, none include
        dp[start].resize(2, 0);
        dp[start][0] = nums[start];
        dp[start + 1].resize(2, 0);
        dp[start + 1][0] = nums[start + 1];
        dp[start + 1][1] = dp[start][0];
        for i in (start + 2)..(end + 1) {
            dp[i].resize(2, 0);
            dp[i][0] = std::cmp::max(dp[i - 2][0], dp[i - 1][1]) + nums[i];
            dp[i][1] = std::cmp::max(dp[i - 1][0], dp[i - 1][1]);
        }
        std::cmp::max(dp[end][0], dp[end][1])
    }

    #[allow(dead_code)]
    pub fn rob_2(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return std::cmp::max(nums[0], nums[1]);
        }
        // [0, nums.len()-2]
        let max1 = Solution::rob_from_to(nums.as_ref(), 0, nums.len() - 2);
        // [1, nums.len()-1]
        let max2 = Solution::rob_from_to(nums.as_ref(), 1, nums.len() - 1);
        std::cmp::max(max1, max2)
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return std::cmp::max(nums[0], nums[1]);
        }

        // include, none include
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(nums.len(), vec![]);
        // include, none include
        dp[0].resize(2, 0);
        dp[0][0] = nums[0];
        dp[1].resize(2, 0);
        dp[1][0] = nums[1];
        dp[1][1] = dp[0][0];
        for i in 2..nums.len() {
            dp[i].resize(2, 0);
            dp[i][0] = std::cmp::max(dp[i - 2][0], dp[i - 1][1]) + nums[i];
            dp[i][1] = std::cmp::max(dp[i - 1][0], dp[i - 1][1]);
        }
        std::cmp::max(dp[nums.len() - 1][0], dp[nums.len() - 1][1])
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit_k(k: i32, prices: Vec<i32>) -> i32 {
        let mut pre: Vec<i32> = vec![];
        let length = ((k + 1) * 2) as usize;
        pre.resize(length, 0);
        for i in 1..(k as usize) {
            pre[i * 2] = -prices[0];
        }
        let mut max_profit = 0;
        let mut tmp_pre: Vec<i32> = vec![];
        tmp_pre.resize(length, 0);
        for i in 1..prices.len() {
            for j in 1..((k + 1) as usize) {
                tmp_pre[2 * j] = std::cmp::max(pre[2 * j], pre[2 * j - 1] - prices[i]);
                tmp_pre[2 * j + 1] = std::cmp::max(pre[2 * j + 1], pre[2 * j] + prices[i]);
            }
            for j in 1..((k + 1) as usize) {
                pre[2 * j] = tmp_pre[2 * j];
                pre[2 * j + 1] = tmp_pre[2 * j + 1];
                max_profit = std::cmp::max(max_profit, pre[2 * j + 1]);
            }
        }
        max_profit
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit_2(prices: Vec<i32>) -> i32 {
        let mut pre = (-prices[0], 0, -prices[0], 0);
        for i in 1..prices.len() {
            let b1 = std::cmp::max(pre.0, -prices[i]);
            let s1 = std::cmp::max(pre.1, pre.0 + prices[i]);
            let b2 = std::cmp::max(pre.2, pre.1 - prices[i]);
            let s2 = std::cmp::max(pre.3, pre.2 + prices[i]);
            pre = (b1, s1, b2, s2);
        }
        std::cmp::max(pre.1, pre.3)
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit_fee(prices: Vec<i32>, fee: i32) -> i32 {
        // solled, unsold
        let mut pre = (0, -prices[0]);
        //println!("i:{}, pre:{:?}", 0, pre);
        for i in 1..prices.len() {
            // sold, fee would payed when sell
            let sold = std::cmp::max(pre.0, pre.1 + prices[i] - fee);
            let keep = std::cmp::max(pre.0 - prices[i], pre.1);
            pre.0 = sold;
            pre.1 = keep;
            //println!("i:{}, pre:{:?}", i, pre);
        }
        if pre.0 < 0 && pre.1 < 0 {
            return 0;
        }
        std::cmp::max(pre.0, pre.1)
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit_cold(prices: Vec<i32>) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(prices.len(), vec![]);
        // own a stock; has sold last day; sold days ago
        dp[0].resize(3, 0);
        dp[0][0] = -prices[0];
        for i in 1..prices.len() {
            dp[i].resize(3, 0);
            dp[i][0] = std::cmp::max(dp[i - 1][0], dp[i - 1][2] - prices[i]);
            dp[i][1] = dp[i - 1][0] + prices[i];
            dp[i][2] = std::cmp::max(dp[i - 1][1], dp[i - 1][2]);
        }

        std::cmp::max(dp[prices.len() - 1][1], dp[prices.len() - 1][2])
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut dp_max, mut dp_min) = (vec![], vec![]);
        dp_max.resize(nums.len(), 0);
        dp_min.resize(nums.len(), 0);
        let (mut max, mut min, mut sum) = (nums[0], nums[0], nums[0]);
        if nums.len() == 1 {
            return nums[0];
        }
        for i in 1..nums.len() {
            dp_max[i] = std::cmp::max(dp_max[i - 1] + nums[i], nums[i]);
            dp_min[i] = std::cmp::min(dp_min[i - 1] + nums[i], nums[i]);
            max = std::cmp::max(dp_max[i], max);
            min = std::cmp::min(dp_min[i], min);
            sum += nums[i];
        }
        let may_max = sum - min;
        if may_max == 0 {
            return max;
        }
        std::cmp::max(max, may_max)
    }
}

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
        for i in 0..dp.len() {
            println!("{:?}", dp[i]);
        }
        max
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut pre_min, mut max) = (prices[0], 0);
        for i in 1..prices.len() {
            if prices[i] < pre_min {
                pre_min = prices[i];
                continue;
            }
            let profit = prices[i] - pre_min;
            max = std::cmp::max(max, profit);
        }

        max
    }
}

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

impl Solution {
    #[allow(dead_code)]
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        // pre: (postive_len,  negetive_len)
        let (mut pre, mut max) = ((0, 0), 0);
        for i in 0..nums.len() {
            max = std::cmp::max(pre.0, max);
            if nums[i] == 0 {
                pre = (0, 0);
                continue;
            }
            if nums[i] > 0 {
                pre.0 += 1;
                pre.1 = if pre.1 > 0 { pre.1 + 1 } else { 0 };
                println!("i:{}, pre:{:?}, max:{}", i, pre, max);
                continue;
            }
            // nums[i] < 0
            let pos = if pre.1 > 0 { pre.1 + 1 } else { 0 };
            let neg = if pre.0 > 0 { pre.0 + 1 } else { 1 };
            pre = (pos, neg);
            println!("i:{}, pre:{:?}, max:{}", i, pre, max);
        }
        println!("pre:{:?}, max:{}", pre, max);
        std::cmp::max(pre.0, max)
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut pre, mut max) = ((nums[0], nums[0]), nums[0]);
        for i in 1..nums.len() {
            let (a, b) = (pre.0 * nums[i], pre.1 * nums[i]);
            pre.0 = std::cmp::min(std::cmp::min(a, b), nums[i]);
            pre.1 = std::cmp::max(std::cmp::max(a, b), nums[i]);
            let t_max = std::cmp::max(pre.0, pre.1);
            max = std::cmp::max(t_max, max);
        }
        max
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut pre, mut max) = (nums[0], nums[0]);
        for i in 1..nums.len() {
            pre = std::cmp::max(pre + nums[i], nums[i]);
            max = std::cmp::max(max, pre);
        }
        max
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let (mut pre, mut max) = ((nums[0], nums[0]), nums[0].abs());
        for i in 1..nums.len() {
            pre.0 = std::cmp::min(pre.0 + nums[i], nums[i]);
            pre.1 = std::cmp::max(pre.1 + nums[i], nums[i]);
            let t_max = std::cmp::max(pre.0.abs(), pre.1.abs());
            max = std::cmp::max(max, t_max);
        }
        max
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        cost.push(0);
        let mut dp: Vec<i32> = vec![];
        dp.resize(cost.len(), 0);
        dp[0] = cost[0];
        dp[1] = cost[1];
        if cost.len() <= 3 {
            return std::cmp::min(dp[0] + cost[2], dp[1]);
        }
        for i in 0..cost.len() - 2 {
            dp[i + 2] = std::cmp::min(dp[i], dp[i + 1]) + cost[i + 2];
        }
        println!("{:?}", dp);
        dp[cost.len() - 1]
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let (b1, b2) = (s1.as_bytes(), s2.as_bytes());
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(b1.len() + 1, vec![]);
        dp[0].resize(b2.len() + 1, 0);
        dp[0][0] = 0;
        // init dp[0]
        for i in 1..(b2.len() + 1) {
            dp[0][i] = dp[0][i - 1] + b2[i - 1] as i32;
        }
        // check dp
        for i in 1..(b1.len() + 1) {
            dp[i].resize(b2.len() + 1, 0);
            dp[i][0] = dp[i - 1][0] + b1[i - 1] as i32;
            for j in 1..(b2.len() + 1) {
                if b1[i - 1] == b2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                    continue;
                }
                dp[i][j] = std::cmp::min(
                    dp[i][j - 1] + (b2[j - 1] as i32),
                    dp[i - 1][j] + (b1[i - 1] as i32),
                );
            }
        }
        dp[b1.len()][b2.len()]
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (byt_1, byt_2) = (word1.as_bytes(), word2.as_bytes());
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(byt_1.len() + 1, vec![]);
        // init dp[0]
        dp[0].resize(byt_2.len() + 1, 0);
        for i in 0..(byt_2.len() + 1) {
            dp[0][i] = i as i32;
        }
        for i in 1..(byt_1.len() + 1) {
            dp[i].resize(byt_2.len() + 1, 0);
            dp[i][0] = i as i32;
            for j in 1..(byt_2.len() + 1) {
                if byt_1[i - 1] == byt_2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                    continue;
                }
                dp[i][j] = std::cmp::min(dp[i - 1][j], dp[i][j - 1]) + 1;
            }
        }

        dp[byt_1.len()][byt_2.len()]
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s_b = s.as_bytes();
        let mut dp: Vec<Vec<i32>> = vec![];
        dp.resize(s_b.len(), vec![]);
        for i in 0..s_b.len() {
            dp[i] = vec![];
            dp[i].resize(s_b.len(), 0);
        }
        let mut i = s_b.len() - 1;
        loop {
            dp[i][i] = 1;
            let mut j = i + 1;
            while j < s_b.len() {
                if s_b[i] == s_b[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j - 1]);
                }
                j += 1;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        dp[0][s_b.len() - 1]
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (s1_b, s2_b, s3_b) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut dp: Vec<Vec<bool>> = vec![];
        dp.resize(s1_b.len() + 1, vec![]);
        // init dp[i][0]
        dp[0] = vec![];
        dp[0].resize(s2_b.len() + 1, false);
        dp[0][0] = if s3_b.len() == 0 { true } else { false };
        for i in 1..(s1_b.len() + 1) {
            dp[i] = vec![];
            dp[i].resize(s2_b.len() + 1, false);
            if dp[i - 1][0] && s1_b[i - 1] == s3_b[i - 1] {
                dp[i][0] = true;
            }
        }
        // init dp[0][i]
        for i in 1..(s2_b.len() + 1) {
            if dp[0][i - 1] && s2_b[i - 1] == s3_b[i - 1] {
                dp[0][i] = true;
            }
        }
        // check s1 and s2
        for i in 1..(s1_b.len() + 1) {
            for j in 1..(s2_b.len() + 1) {
                if (dp[i - 1][j] && s1_b[i - 1] == s3_b[i + j - 1])
                    || (dp[i][j - 1] && s2_b[j - 1] == s3_b[i + j - 1])
                {
                    dp[i][j] = true;
                }
            }
        }
        //Solution::print_arr(&dp);
        dp[s1_b.len()][s2_b.len()]
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn print_arr(dp: &Vec<Vec<bool>>) {
        for i in 0..dp.len() {
            println!("{:?}", dp[i]);
        }
    }
}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut dp: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut longest = 0;
        let mut i = 0;
        loop {
            if i >= nums.len() {
                break;
            }
            let iter = nums[i];
            if dp.get(&iter).is_some() {
                i += 1;
                continue;
            }
            let l;
            if let Some(v) = dp.get(&(iter - 1)) {
                l = v.clone();
            } else {
                l = 0;
            }
            let r;
            if let Some(v) = dp.get(&(iter + 1)) {
                r = v.clone();
            } else {
                r = 0;
            }
            let mut dv = l + r + 1;
            if let Some(v) = dp.get(&iter) {
                if *v > dv {
                    dv = v.clone();
                }
            }
            dp.insert(iter, dv);
            dp.insert(iter - l, dv);
            dp.insert(iter + r, dv);
            i += 1;
            if longest < dv {
                longest = dv;
            }
        }
        longest
    }
}

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

        println!("{:?}", dp);
        max_s * max_s
    }
}

impl Solution {
    pub fn ways_to_step(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        let mut i = 1;
        'ac: loop {
            if i as i32 > n {
                break 'ac;
            }
            if i >= 1 {
                dp[i] += dp[i - 1];
            }
            if i >= 2 {
                dp[i] += dp[i - 2];
            }
            if i >= 3 {
                dp[i] += dp[i - 3];
            }
            i += 1;
        }
        dp[i - 1]
    }
}
