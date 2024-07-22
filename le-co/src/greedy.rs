use std::{cmp::Ordering, collections::HashMap};

pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn is_match_r(s: String, p: String) -> bool {
        let (s_b, p_b) = (s.as_bytes(), p.as_bytes());
        let mut dp: Vec<Vec<bool>> = vec![];
        dp.resize(s_b.len() + 1, vec![]);
        let dot = b'.';
        let star = b'*';
        // init
        dp[0] = vec![];
        dp[0].resize(p_b.len() + 1, false);
        dp[0][0] = true;
        for i in 1..(p_b.len() + 1) {
            if p_b[i - 1] == star {
                dp[0][i] = dp[0][i - 2];
            }
        }
        for i in 1..(s_b.len() + 1) {
            dp[i] = vec![];
            dp[i].resize(p_b.len() + 1, false);
            for j in 1..(p_b.len() + 1) {
                if p_b[j - 1] == dot {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p_b[j - 1] == star {
                    if s_b[i - 1] == p_b[j - 2] || p_b[j - 2] == dot {
                        dp[i][j] = dp[i - 1][j] | dp[i][j - 2];
                    } else {
                        dp[i][j] = dp[i][j - 2];
                    }
                } else {
                    // not special character
                    if s_b[i - 1] == p_b[j - 1] {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                }
            }
        }
        for i in 0..(s_b.len() + 1) {
            for j in 0..(p_b.len() + 1) {
                print!("{}\t", dp[i][j]);
            }
            println!("");
        }
        dp[s_b.len()][p_b.len()]
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn is_match(s: String, p: String) -> bool {
        let (s_byt, p_byt) = (s.as_bytes(), p.as_bytes());
        let sing = b'?';
        let mult = b'*';
        let mut dp: Vec<Vec<bool>> = vec![];
        dp.resize(s_byt.len() + 1, vec![]);
        // init 0;
        let mut cap: Vec<bool> = vec![];
        cap.resize(p_byt.len() + 1, false);
        dp[0] = cap;
        dp[0][0] = true;
        for i in 1..(p_byt.len() + 1) {
            if p_byt[i - 1] == mult {
                dp[0][i] = true;
                continue;
            }
            break;
        }
        for i in 1..(s_byt.len() + 1) {
            let mut cap: Vec<bool> = vec![];
            cap.resize(p_byt.len() + 1, false);
            dp[i] = cap;
            for j in 1..(p_byt.len() + 1) {
                if s_byt[i - 1] == p_byt[j - 1] || p_byt[j - 1] == sing {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p_byt[j - 1] == mult {
                    dp[i][j] = dp[i - 1][j] | dp[i][j - 1];
                }
            }
        }
        dp[s_byt.len()][p_byt.len()]
    }
}

impl Solution {
    #[allow(dead_code)]
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

impl Solution {
    #[allow(dead_code)]
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let (mut even, mut odd) = (0, 0);
        for p in 0..position.len() {
            let pos = position[p] % 2;
            match pos {
                0 => even += 1,
                1 => odd += 1,
                _ => even += 0,
            }
        }
        if even > odd {
            return odd;
        }
        even
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn check(nums: &Vec<i32>, x: i32, m: i32) -> bool {
        let (mut sum, mut cnt) = (0, 1);
        for i in 0..nums.len() {
            if sum + nums[i] > x {
                cnt += 1;
                sum = nums[i];
            } else {
                sum += nums[i];
            }
        }
        cnt <= m
    }

    #[allow(dead_code)]
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let (mut left, mut right) = (0, 0);
        for i in 0..nums.len() {
            right += nums[i];
            if left < nums[i] {
                left = nums[i];
            }
        }
        while left < right {
            let mid = left + (right - left) / 2;
            if Solution::check(&nums, mid, m) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let mut max_seq = vec![];
        for len1 in 0..k {
            println!(
                "len1:{}, nums1:{}, nums2:{}",
                len1,
                nums1.len(),
                nums2.len()
            );
            if (len1 as usize) > nums1.len() {
                continue;
            }
            if (k - len1) as usize > nums2.len() {
                continue;
            }
            let seq1 = Solution::find_max_subsequence(&nums1, len1);
            println!("seq1:{:?}", seq1);
            let seq2 = Solution::find_max_subsequence(&nums2, k - len1);
            println!("seq2:{:?}", seq2);
            let merge = Solution::merge_seq(&seq1, &seq2);
            println!("merge:{:?}", merge);
            if max_seq.len() == 0 {
                max_seq = merge;
                continue;
            }
            let cmp = Solution::compare_seq(&merge, 0, &max_seq, 0);
            match cmp {
                Ordering::Greater => {
                    max_seq = merge;
                }
                _ => {}
            }
        }
        max_seq
    }

    pub fn find_max_subsequence(nums: &Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![];
        if k == 0 {
            return res;
        }
        res.resize(k as usize, 0);
        let mut top = -1;
        let mut remain = nums.len() - (k as usize);
        for pos in 0..nums.len() {
            println!("pos:{}, res:{:?}", pos, res);
            while (top >= 0) && remain > 0 && res[top as usize] < nums[pos] {
                top -= 1;
                remain -= 1;
            }
            if top < k - 1 {
                top += 1;
                res[top as usize] = nums[pos];
            } else {
                remain -= 1;
            }
        }
        res
    }

    pub fn compare_seq(seq1: &Vec<i32>, p1: usize, seq2: &Vec<i32>, p2: usize) -> Ordering {
        let (mut p1, mut p2) = (p1, p2);
        while p1 < seq1.len() && p2 < seq2.len() {
            if seq1[p1] < seq2[p2] {
                return Ordering::Less;
            }
            if seq1[p1] > seq2[p2] {
                return Ordering::Greater;
            }
            p1 += 1;
            p2 += 1;
        }
        if p1 == seq1.len() && p2 == seq2.len() {
            return Ordering::Equal;
        }
        if p1 < seq1.len() {
            return Ordering::Greater;
        }
        Ordering::Less
    }

    pub fn merge_seq(seq1: &Vec<i32>, seq2: &Vec<i32>) -> Vec<i32> {
        let (mut p1, mut p2) = (0 as usize, 0 as usize);
        let mut res = vec![];
        'merge: loop {
            if p1 >= seq1.len() && p2 >= seq2.len() {
                break 'merge;
            }
            let cmp = Solution::compare_seq(&seq1, p1, &seq2, p2);
            match cmp {
                Ordering::Less => {
                    res.push(seq2[p2]);
                    p2 += 1;
                }
                _ => {
                    res.push(seq1[p1]);
                    p1 += 1;
                }
            }
        }
        res
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 1;
        let mut points_sort = points.to_vec();
        points_sort.sort_by(|x, y| x[0].cmp(&y[0]));
        println!("after sorted {:?}", points_sort);
        #[allow(unused_assignments)]
        let mut arrow = &points_sort[0];
        let mut point = points_sort[0][1];
        for pos in 0..points_sort.len() {
            println!("point: {}", point);
            if points_sort[pos][1] > point {
                ans += 1;
                arrow = &points_sort[pos];
                point = arrow[1];
                continue;
            }
            if points_sort[pos][1] < point {
                point = points_sort[pos][1];
            }
        }

        ans
    }
}

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

impl Solution {
    #[allow(dead_code)]
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_by(|&x, &y| {
            let (x, y) = (x as i64, y as i64);
            let (mut sx, mut sy) = (10 as i64, 10 as i64);
            loop {
                if sx <= x {
                    sx *= 10;
                    continue;
                }
                break;
            }
            loop {
                if sy <= y {
                    sy *= 10;
                    continue;
                }
                break;
            }
            if x * sy + y > y * sx + x {
                return Ordering::Less;
            }
            return Ordering::Greater;
        });
        if nums[0] == 0 {
            return String::from("0");
        }
        println!("{:?}", nums);
        let mut result = String::from("");
        for v in nums.iter() {
            result += &String::from(v.to_string());
        }
        result
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let (mut up, mut down) = (1, 1);
        nums.windows(2).for_each(|x| {
            if x[0] > x[1] {
                up = down + 1;
            }
            if x[0] < x[1] {
                down = up + 1;
            }
        });
        std::cmp::max(up, down)
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut max_perimter = 0;
        let mut pos = nums.len() - 1;
        while pos >= 2 {
            // right donnot need to move, why?
            let (mut left, right) = (0 as usize, pos - 1);
            while left < right {
                if nums[pos] - nums[left] >= nums[right] {
                    left += 1;
                    continue;
                }
                if nums[pos] - nums[right] >= nums[left] {
                    left += 1;
                    continue;
                }
                if nums[left] + nums[right] <= nums[pos] {
                    left += 1;
                    continue;
                }
                let permiter = nums[pos] + nums[left] + nums[right];
                if permiter > max_perimter {
                    max_perimter = permiter;
                }
                left += 1;
            }
            pos -= 1;
        }
        max_perimter
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        // change count for 5, 10, 20
        let mut change = (0, 0, 0);
        fn find_change(change: &mut (i32, i32, i32), need: i32) -> bool {
            if change.0 == 0 && change.1 == 0 {
                return false;
            }
            match need {
                // pay 20, need change 15
                15 => {
                    if change.1 > 0 && change.0 > 0 {
                        change.1 -= 1;
                        change.0 -= 1;
                    } else if change.0 >= 3 {
                        change.0 -= 3;
                    } else {
                        return false;
                    }
                }
                // pay 10, need change 5
                5 => {
                    if change.0 > 0 {
                        change.0 -= 1;
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
            true
        }

        fn set_change(change: &mut (i32, i32, i32), pay: i32) -> bool {
            match pay {
                5 => change.0 += 1,
                10 => change.1 += 1,
                20 => change.2 += 1,
                _ => return false,
            }
            true
        }
        for i in 0..bills.len() {
            if bills[i] > 5 {
                println!("bill: {}, change:{:?}", bills[i], change);
                let ok = find_change(&mut change, bills[i] - 5);
                if !ok {
                    return false;
                }
            }
            set_change(&mut change, bills[i]);
        }
        true
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() == 1 {
            return 1;
        }
        let mut candy_num = 1;
        let mut status = (1, 1); // inc, dec
        let mut inc_max = 0;
        for pos in 1..ratings.len() {
            if ratings[pos] == ratings[pos - 1] {
                candy_num += 1;
                println!("pos: {}, candy_num: {}", pos, candy_num);
                status.0 = 1;
                status.1 = 1;
            } else if ratings[pos] > ratings[pos - 1] {
                status.0 += 1;
                candy_num += status.0;
                status.1 = 1;
                inc_max = status.0;
                println!("pos: {}, candy_num: {}", pos, candy_num);
            } else if ratings[pos] < ratings[pos - 1] {
                status.0 = 1;
                status.1 += 1;
                if status.1 >= inc_max {
                    candy_num += 1;
                }
                candy_num += status.1;
                println!("pos: {}, candy_num: {}", pos, candy_num);
            }
        }
        candy_num
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut max_sum = 0;
        let mut pos = 0 as usize;
        while pos < nums.len() {
            max_sum += nums[pos];
            pos += 2;
        }
        return max_sum;
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut w = w;
        let mut cap_pro: Vec<(&i32, &i32)> = capital.iter().zip(profits.iter()).collect();
        cap_pro.sort_by(|a, b| (a.0.cmp(&b.0)));
        let mut heap = std::collections::BinaryHeap::new();
        let mut j = 0 as usize;
        for _i in 0..k {
            while j < cap_pro.len() && *cap_pro[j].0 <= w {
                heap.push(*cap_pro[j].1);
                j += 1;
            }
            let max_profit = heap.pop();
            if max_profit.is_some() {
                w += max_profit.unwrap();
            }
        }
        w
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort();
        s.sort();
        let mut content = 0;
        let (mut g_p, mut s_p) = (0 as usize, 0 as usize);
        while g_p < g.len() && s_p < s.len() {
            while s_p < s.len() {
                if g[g_p] <= s[s_p] {
                    content += 1;
                    break;
                }
                s_p += 1;
            }
            g_p += 1;
        }

        content
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }
        let mut triplet: (usize, usize, usize) = (0, 0, 0);
        let mut pos = 0 as usize;
        while pos < nums.len() {
            // check least
            if nums[triplet.0] > nums[pos] {
                triplet.0 = pos;
                pos += 1;
                continue;
            }
            if nums[triplet.0] == nums[pos] {
                pos += 1;
                continue;
            }
            if triplet.1 == 0 {
                triplet.1 = pos;
                pos += 1;
                continue;
            }
            if nums[triplet.1] < nums[pos] {
                triplet.2 = pos;
                println!("{:?}", triplet);
                return true;
            } else if nums[triplet.1] == nums[pos] {
                pos += 1;
                continue;
            }
            // nums[triplet.1] > nums[pos]
            triplet.1 = pos;
            pos += 1;
        }

        false
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicate_letters(s: String) -> String {
        let s_b = s.into_bytes();
        let mut left: Vec<i32> = vec![];
        left.resize(26, 0);
        for pos in 0..s_b.len() {
            let p = s_b[pos] - b'a';
            left[p as usize] += 1;
        }
        let mut stack: Vec<u8> = vec![];
        let mut is_in: Vec<bool> = vec![];
        is_in.resize(26, false);
        for pos in 0..s_b.len() {
            let abs = (s_b[pos] - b'a') as usize;
            if !is_in[abs as usize] {
                while (stack.len() > 0) && (stack[stack.len() - 1] > s_b[pos]) {
                    let last_abs = (stack[stack.len() - 1] - b'a') as usize;
                    if left[last_abs] == 0 {
                        break;
                    }
                    stack.pop();
                    is_in[last_abs] = false;
                }
                stack.push(s_b[pos]);
                is_in[abs] = true;
            }
            left[abs] -= 1;
        }
        String::from_utf8(stack[0..stack.len()].to_vec()).unwrap()
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome(s: String) -> i32 {
        let mut b_c: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            let count = b_c.entry(c).or_insert(0);
            *count += 1;
        }
        let mut has_single = false;
        let mut length = 0;
        for ele in b_c {
            if ele.1 % 2 == 1 {
                has_single = true;
                length += ele.1 - 1;
            } else {
                length += ele.1;
            }
        }
        if has_single {
            length += 1;
        }

        length
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        people.sort_by(|a, b| (&b[0]).cmp(&a[0]));
        let mut result: Vec<Vec<i32>> = vec![];
        let mut last = 0 as usize;
        let mut pos = 0 as usize;
        while pos < people.len() {
            if people[pos][0] == people[last][0] {
                pos += 1;
                continue;
            }
            // reorder
            people[last..pos].sort_by(|a, b| (&b[1]).cmp(&a[1]));
            while last < pos {
                let idx = people[last][1] as usize;
                result.insert(idx, vec![people[last][0], people[last][1]]);
                last += 1;
            }
            pos += 1;
        }
        people[last..pos].sort_by(|a, b| (&a[1]).cmp(&b[1]));
        while last < pos {
            let idx = people[last][1] as usize;
            result.insert(idx, vec![people[last][0], people[last][1]]);
            last += 1;
        }
        result
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .zip(prices[1..].iter())
            .fold(
                0,
                |acc, (buy, sell)| if buy < sell { acc + sell - buy } else { acc },
            )
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut gas_sum = 0;
        let mut start_sum = 0;
        let mut start = 0;
        for i in 0..gas.len() {
            start_sum += gas[i] - cost[i];
            gas_sum += gas[i] - cost[i];
            if start_sum < 0 {
                start = ((i + 1) % gas.len()) as i32;
                start_sum = 0;
            }
        }
        if gas_sum >= 0 {
            return start;
        }
        return -1;
    }
}
