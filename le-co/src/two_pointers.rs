impl Solution {
    #[allow(dead_code)]
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut new = nums.clone();
        let (mut pos, mut neg) = (0 as usize, 1 as usize);
        let mut p = 0 as usize;
        while p < nums.len() {
            if nums[p] < 0 {
                new[neg] = nums[p];
                neg += 2;
            } else {
                new[pos] = nums[p];
                pos += 2;
            }
            p += 1;
        }
        new
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut new: Vec<i32> = nums.clone();
        let (mut less, mut great) = (0 as usize, 0 as usize);
        let mut pos = 0 as usize;
        while pos < nums.len() {
            if nums[pos] > pivot {
                great += 1;
            }
            if nums[pos] < pivot {
                less += 1;
            }
            pos += 1;
        }
        let (mut l_p, mut e_p, mut g_p) = (0 as usize, less, nums.len() - great);
        pos = 0;
        while pos < nums.len() {
            if nums[pos] == pivot {
                new[e_p] = nums[pos];
                e_p += 1;
            }
            if nums[pos] < pivot {
                new[l_p] = nums[pos];
                l_p += 1;
            }
            if nums[pos] > pivot {
                new[g_p] = nums[pos];
                g_p += 1;
            }
            pos += 1;
        }
        new
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut head = head;
        // find length
        let length = Solution::list_length(head.as_ref());
        let mid_idx = length / 2 - 1;
        // move to the middle
        let h_mid = Solution::move_to(head.as_mut(), mid_idx);
        // reverse [mid, None)
        let h_pair = Solution::reverse(h_mid);
        // check sum max
        let mut sum_max = 0 as i32;
        let mut node = head.as_ref();
        let mut pair = h_pair.as_ref();
        while pair.is_some() {
            let node_node = node.unwrap();
            let pair_node = pair.unwrap();
            sum_max = std::cmp::max(sum_max, node_node.val + pair_node.val);
            node = node_node.next.as_ref();
            pair = pair_node.next.as_ref();
        }
        sum_max
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn list_length(head: Option<&Box<ListNode>>) -> i32 {
        let mut length = 0;
        let mut node = head;
        while node.is_some() {
            let node_node = node.unwrap();
            node = node_node.next.as_ref();
            length += 1;
        }
        length
    }

    #[allow(dead_code)]
    pub fn move_to(head: Option<&mut Box<ListNode>>, idx: i32) -> Option<&mut Box<ListNode>> {
        let mut cur = 0;
        let mut node = head;
        while cur < idx {
            if node.is_none() {
                return node;
            }
            let node_node = node.unwrap();
            node = node_node.next.as_mut();
            cur += 1;
        }
        node
    }

    #[allow(dead_code)]
    pub fn reverse(head: Option<&mut Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_ref().is_none() {
            return None;
        }
        let mut new_node = ListNode::new(head.as_ref().unwrap().val);
        let mut node = head.unwrap().next.as_ref();
        while node.is_some() {
            let node_node = node.unwrap();
            new_node = ListNode {
                val: node_node.val,
                next: Some(Box::new(new_node)),
            };
            node = node_node.next.as_ref();
        }
        Some(Box::new(new_node))
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn first_palindrome(words: Vec<String>) -> String {
        fn is_palindrome(word: &String) -> bool {
            let w_b = word.as_bytes();
            let (mut start, mut end) = (0 as usize, w_b.len() - 1);
            while start < end {
                if w_b[start] != w_b[end] {
                    return false;
                }
                start += 1;
                end -= 1;
            }
            true
        }
        let mut pos = 0 as usize;
        while pos < words.len() {
            if is_palindrome(&words[pos]) {
                return words[pos].clone();
            }
            pos += 1;
        }
        String::from("")
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut w_b = word.into_bytes();
        let (mut start, mut end) = (0 as usize, 0 as usize);
        while start < w_b.len() {
            if char::from(w_b[start]) == ch {
                end = start;
                break;
            }
            start += 1;
        }
        start = 0 as usize;
        while start < end {
            w_b.swap(start, end);
            start += 1;
            end -= 1;
        }

        String::from_utf8(w_b).unwrap()
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let w_b1 = word1.into_bytes();
        let w_b2 = word2.into_bytes();
        let mut new_word: Vec<u8> = vec![];
        let (mut p1, mut p2) = (0 as usize, 0 as usize);
        while p1 < w_b1.len() && p2 < w_b2.len() {
            new_word.push(w_b1[p1]);
            new_word.push(w_b2[p2]);
            p1 += 1;
            p2 += 1;
        }
        while p1 < w_b1.len() && p2 >= w_b2.len() {
            new_word.push(w_b1[p1]);
            p1 += 1;
        }
        while p1 >= w_b1.len() && p2 < w_b2.len() {
            new_word.push(w_b2[p2]);
            p2 += 1;
        }

        String::from_utf8(new_word).unwrap()
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut num = 0 as usize;
        // find 0 num
        let mut pos = 0 as usize;
        let mut length = arr.len() - 1;
        while pos < arr.len() - num {
            if arr[pos] == 0 {
                if pos + num == length {
                    arr[length] = 0;
                    length -= 1;
                    break;
                }
                num += 1;
            }
            pos += 1;
        }
        // start to move
        pos = length - num;
        let mut end = length;
        while num > 0 {
            if arr[pos] == 0 {
                arr[end] = 0;
                arr[end - 1] = 0;
                end -= 2;
                pos -= 1;
                num -= 1;
                continue;
            }
            arr[end] = arr[pos];
            pos -= 1;
            end -= 1;
        }
        return;
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut dp: Vec<i32> = Vec::with_capacity(words.len());
        fn is_predecessor(words: &Vec<String>, pre: usize, cur: usize) -> bool {
            let pre_b = words[pre].as_bytes();
            let cur_b = words[cur].as_bytes();
            let mut is_decessor = false;
            let (mut p_p, mut p_c) = (0 as usize, 0 as usize);
            while p_p < pre_b.len() || p_c < cur_b.len() {
                if p_p == pre_b.len() {
                    if is_decessor == false {
                        is_decessor = true;
                        p_c += 1;
                        continue;
                    }
                    return false;
                }
                if p_c == cur_b.len() {
                    return false;
                }
                if pre_b[p_p] != cur_b[p_c] {
                    if !is_decessor {
                        is_decessor = true;
                        p_c += 1;
                        continue;
                    }
                    return false;
                }
                p_p += 1;
                p_c += 1;
            }
            true
        }
        dp.push(1);
        let mut max_chain = 1;
        let mut pos = 1 as usize;
        while pos < words.len() {
            if is_predecessor(&words, pos - 1, pos) {
                dp.push(dp[pos - 1] + 1);
                max_chain = std::cmp::max(dp[pos], max_chain);
            } else {
                dp.push(1);
            }
            pos += 1;
        }
        max_chain
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut result: Vec<bool> = vec![];
        let p_b = pattern.as_bytes();
        for query in queries.iter() {
            let q_b = query.as_bytes();
            let (mut q, mut p) = (0 as usize, 0 as usize);
            let mut res = true;
            while q < q_b.len() {
                if q_b[q] >= b'A' && q_b[q] <= b'Z' {
                    if p >= p_b.len() {
                        res = false;
                        break;
                    }
                    if q_b[q] == p_b[p] {
                        q += 1;
                        p += 1;
                        continue;
                    } else {
                        res = false;
                        break;
                    }
                }
                if p < p_b.len() && q_b[q] == p_b[p] {
                    q += 1;
                    p += 1;
                    continue;
                }
                q += 1;
            }
            if p < p_b.len() {
                res = false;
            }
            result.push(res);
        }
        result
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut new_list: Vec<Vec<i32>> = vec![];
        let (mut first, mut second) = (0 as usize, 0 as usize);
        while first < first_list.len() && second < second_list.len() {
            if first_list[first][0] > second_list[second][1] {
                second += 1;
                continue;
            }
            if first_list[first][1] < second_list[second][0] {
                first += 1;
                continue;
            }
            let v_s = std::cmp::max(first_list[first][0], second_list[second][0]);
            let v_e = std::cmp::min(first_list[first][1], second_list[second][1]);
            if second_list[second][1] > first_list[first][1] {
                first += 1;
            } else {
                second += 1;
            }
            new_list.push(vec![v_s, v_e]);
        }
        new_list
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut squares: Vec<i32> = vec![];
        let (mut left, mut right) = (0 as usize, nums.len() - 1);
        while left < right {
            let right_value = nums[right] * nums[right];
            let left_value = nums[left] * nums[left];
            if left_value > right_value {
                squares.push(left_value);
                left += 1;
                continue;
            }
            squares.push(right_value);
            right -= 1;
        }
        squares.push(nums[left] * nums[left]);
        squares.reverse();
        squares
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn di_string_match(s: String) -> Vec<i32> {
        let s_b = s.into_bytes();
        let mut s_v: Vec<i32> = vec![];
        let (mut lo, mut hi) = (0 as i32, s_b.len() as i32);
        let mut pos = 0 as usize;
        while pos < s_b.len() {
            let b = s_b[pos];
            if char::from(b) == 'I' {
                s_v.push(lo);
                lo += 1;
            }
            if char::from(b) == 'D' {
                s_v.push(hi);
                hi -= 1;
            }
            pos += 1;
        }
        s_v.push(lo);

        s_v
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut ab_odd, mut ab_even) = (1 as usize, 0 as usize);
        while ab_odd < nums.len() && ab_even < nums.len() {
            // find next ab_odd
            while ab_odd < nums.len() {
                if nums[ab_odd] % 2 == 0 {
                    break;
                }
                ab_odd += 2;
            }
            while ab_even < nums.len() {
                if nums[ab_even] % 2 == 1 {
                    break;
                }
                ab_even += 2;
            }
            if ab_odd > nums.len() || ab_even > nums.len() {
                break;
            }
            if nums[ab_odd] % 2 == 1 || nums[ab_even] % 2 == 0 {
                break;
            }
            nums.swap(ab_odd, ab_even);
        }
        nums
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_only_letters(s: String) -> String {
        let mut s_b = s.into_bytes();
        let (mut left, mut right) = (0 as usize, s_b.len() - 1);
        while left < right {
            // find left
            while left < right {
                if s_b[left].is_ascii_alphabetic() {
                    break;
                }
                left += 1;
            }
            while right > left {
                if s_b[right].is_ascii_alphabetic() {
                    break;
                }
                right -= 1;
            }
            s_b.swap(left, right);
            left += 1;
            right -= 1;
        }
        std::str::from_utf8(&s_b).unwrap().to_string()
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut top_even = 0 as usize;
        let mut pos = 0 as usize;
        while pos < nums.len() {
            if nums[pos] % 2 == 0 {
                nums.swap(top_even, pos);
                top_even += 1;
            }
            pos += 1;
        }

        nums
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn backspace_compare(s: String, t: String) -> bool {
        let (mut s_pos, mut t_pos) = (0 as usize, 0 as usize);
        let (mut s_b, mut t_b) = (s.into_bytes(), t.into_bytes());
        let mut pos = 0 as usize;
        while pos < s_b.len() || pos < t_b.len() {
            if pos < s_b.len() {
                if char::from(s_b[pos]) == '#' {
                    s_pos = if s_pos == 0 { 0 } else { s_pos - 1 };
                } else {
                    s_b.swap(pos, s_pos);
                    s_pos += 1;
                }
            }
            if pos < t_b.len() {
                if char::from(t_b[pos]) == '#' {
                    t_pos = if t_pos == 0 { 0 } else { t_pos - 1 };
                } else {
                    t_b.swap(t_pos, pos);
                    t_pos += 1;
                }
            }
            pos += 1;
        }
        if std::str::from_utf8(s_b.get(0..s_pos).unwrap())
            == std::str::from_utf8(t_b.get(0..t_pos).unwrap())
        {
            return true;
        }

        false
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn invert(v: i32) -> i32 {
            match v {
                0 => return 1,
                1 => return 0,
                _ => return 0,
            }
        }
        let n = image.len();
        println!("{}", n);
        let mut im: Vec<Vec<i32>> = vec![];
        for i in 0..n {
            let mut v: Vec<i32> = vec![];
            let mut j = n - 1;
            loop {
                v.push(invert(image[i][j]));
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            im.push(v);
        }
        im
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let s_b = s.into_bytes();
        fn find_next(s_b: &Vec<u8>, mut begin: usize, c: char) -> usize {
            if begin == s_b.len() {
                return begin;
            }
            while begin < s_b.len() {
                if char::from(s_b[begin]) == c {
                    break;
                }
                begin += 1;
            }
            return begin;
        }
        let mut pos = 0 as usize;
        let mut cur = find_next(&s_b, pos, c) as i32;
        let mut cur_next = find_next(&s_b, (cur + 1) as usize, c) as i32;
        if cur_next == (s_b.len() as i32) {
            cur_next = cur;
        }
        let mut dist: Vec<i32> = vec![];
        while pos < s_b.len() {
            let pos_i = pos as i32;
            let cur_dist = (cur - pos_i).abs();
            let next_dist = (cur_next - pos_i).abs();
            if next_dist < cur_dist {
                cur = cur_next;
                cur_next = find_next(&s_b, (cur + 1) as usize, c) as i32;
                if cur_next == s_b.len() as i32 {
                    cur_next = cur;
                }
            }
            dist.push(std::cmp::min(next_dist, cur_dist));
            pos += 1;
        }
        dist
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn can_transform(start: String, end: String) -> bool {
        let (s_b, e_b) = (start.into_bytes(), end.into_bytes());
        if s_b.len() != e_b.len() {
            return false;
        }
        let (mut s_p, mut e_p) = (0 as usize, 0 as usize);
        let (mut s_x, mut e_x) = (0 as usize, 0 as usize);
        while s_p < s_b.len() || e_p < e_b.len() {
            while s_p < s_b.len() && char::from(s_b[s_p]) == 'X' {
                s_p += 1;
                s_x += 1;
            }
            while e_p < e_b.len() && char::from(e_b[e_p]) == 'X' {
                e_p += 1;
                e_x += 1;
            }
            if s_p == s_b.len() || e_p == e_b.len() {
                break;
            }
            if s_b[s_p] != e_b[e_p] {
                return false;
            }
            if char::from(s_b[s_p]) == 'R' {
                if s_p > e_p {
                    return false;
                }
            } else if char::from(s_b[s_p]) == 'L' {
                if s_p < e_p {
                    return false;
                }
            }
            s_p += 1;
            e_p += 1;
        }
        if s_x != e_x {
            return false;
        }
        true
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut part: Vec<i32> = vec![];
        let mut pos = std::collections::HashMap::new();
        let s_b = s.into_bytes();
        // build pos map
        let mut i = 0 as usize;
        while i < s_b.len() {
            pos.insert(s_b[i], i);
            i += 1;
        }
        let mut start: usize = 0;
        let mut end: usize = 0;
        i = 0;
        while i < s_b.len() {
            end = std::cmp::max(end, *pos.get(&s_b[i]).unwrap());
            // come to the end
            if end == i {
                // push this sentence into vec
                part.push((end - start + 1) as i32);
                i += 1;
                start = i;
                end = i;
                continue;
            }
            i += 1;
        }

        part
    }
}

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }
//use std::cell::RefCell;
//use std::rc::Rc;
//impl Solution {
//    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
//        // find smallest
//        let mut left: Option<Rc<RefCell<TreeNode>>> = None;
//        let mut r_mut = root.as_mut();
//        loop {
//            let r_node = r_mut.unwrap();
//        }
//        // find largest
//        let mut right: Option<Rc<RefCell<TreeNode>>> = None;
//
//        true
//    }
//}

impl Solution {
    #[allow(dead_code)]
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let n: usize = nums.len();

        let (mut low, mut high) = (0 as i32, nums[n - 1] - nums[0]);
        while low < high {
            let m: i32 = (low + high) / 2;
            let mut count: usize = 0;

            let mut i: usize = 0;
            for j in 0..n {
                while nums[j] - nums[i] > m {
                    i += 1;
                }
                count += j - i;
            }
            if count < k as usize {
                low = m + 1;
            } else {
                high = m;
            }
        }

        low
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn judge_square_sum(c: i32) -> bool {
        let c_sqrt = (c as f64).sqrt();
        let (mut a, mut b) = (0 as i32, c_sqrt as i32);
        while a <= b {
            // 2147483600 case would be out of i32
            let square = (a * a) as i64 + (b * b) as i64;
            if square == (c as i64) {
                return true;
            }
            if square < (c as i64) {
                a += 1;
                continue;
            }
            b -= 1;
        }
        false
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut num = 0 as i32;
        let mut nums = nums;
        nums.sort();
        for start in 0..nums.len() {
            for left in start + 1..nums.len() {
                for right in left + 1..nums.len() {
                    if nums[start] + nums[left] > nums[right] {
                        if nums[right] - nums[start] < nums[left] {
                            if nums[right] - nums[left] < nums[start] {
                                num += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        num
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut houses = houses;
        let mut heaters = heaters;
        houses.sort();
        heaters.sort();
        let mut house = 0 as usize;
        let mut heater = 0 as usize;
        let mut radiu = 0 as i32;
        while house < houses.len() {
            let h_radiu = (houses[house] - heaters[heater]).abs();
            let mut h_cur = h_radiu;
            while heater < heaters.len() - 1 {
                let h_next = (houses[house] - heaters[heater + 1]).abs();
                if h_next <= h_cur {
                    h_cur = std::cmp::min(h_cur, h_next);
                    heater += 1;
                } else {
                    break;
                }
            }
            if h_cur != 0 {
                println!("houses: {}, heaters:{}", houses[house], heaters[heater]);
                break;
            }
            if h_cur > radiu {
                radiu = h_cur;
            }
            house += 1;
        }

        radiu
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return false;
        }
        fn next(nums: &Vec<i32>, pos: usize) -> usize {
            ((nums.len() as i32) + (pos as i32) + nums[pos]) as usize % nums.len()
        }
        let mut begin = 0 as usize;
        while begin < nums.len() {
            while begin < nums.len() && begin == next(nums.as_ref(), begin) {
                begin += 1;
            }
            if begin == nums.len() {
                return false;
            }
            println!("begin: {}", begin);
            let (mut slow, mut fast) = (begin, begin);
            // find the entrance
            slow = next(nums.as_ref(), slow);
            fast = next(nums.as_ref(), fast);
            fast = next(nums.as_ref(), fast);
            while slow != fast {
                slow = next(nums.as_ref(), slow);
                fast = next(nums.as_ref(), fast);
                fast = next(nums.as_ref(), fast);
            }
            slow = begin;
            while slow != fast {
                slow = next(nums.as_ref(), slow);
                fast = next(nums.as_ref(), fast);
                fast = next(nums.as_ref(), fast);
            }
            let postive = if nums[slow] > 0 { true } else { false };
            let mut result = true;
            let mut length = 1;
            slow = next(nums.as_ref(), slow);
            while slow != fast {
                if nums[slow] > 0 && !postive {
                    result = false;
                    break;
                }
                if nums[slow] < 0 && postive {
                    result = false;
                    break;
                }
                slow = next(nums.as_ref(), slow);
                length += 1;
            }
            begin += 1;
            if length == 1 {
                continue;
            }
            if result == true {
                return true;
            }
        }
        false
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn count_binary_substrings(s: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let mut re: i32 = 0;
        let mut pre: i32 = 0;
        let mut cur: i32 = 1;
        for i in 1..s_chars.len() {
            if s_chars[i] == s_chars[i - 1] {
                cur += 1;
                continue;
            }
            re += std::cmp::min(pre, cur);
            pre = cur;
            cur = 1;
        }
        return re + std::cmp::min(pre, cur);
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut s_b = s.into_bytes();
        if s_b.len() <= 1 {
            return std::str::from_utf8(&s_b).unwrap().to_string();
        }
        let round = s_b.len() / (2 * k) as usize;
        let mut i = 0 as usize;
        while i < round {
            // in these round, first k [i*2k, i*2k+k),
            let mut left = 2 * i * k;
            let mut right = 2 * i * k + k - 1;
            while left < right {
                s_b.swap(left, right);
                left += 1;
                right -= 1;
            }
            i += 1;
        }
        let rest = s_b.len() - 2 * k * i;
        if rest > k {
            // [2*k*i, 2*k*i+k) reverse
            let mut left = 2 * k * i;
            let mut right = 2 * k * i + k - 1;
            while left < right {
                s_b.swap(left, right);
                left += 1;
                right -= 1;
            }
        } else {
            // [2*k*i, len) reverse
            let mut left = 2 * k * i;
            let mut right = s_b.len() - 1;
            while left < right {
                s_b.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        std::str::from_utf8(&s_b).unwrap().to_string()
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0 as usize, numbers.len() - 1);
        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum > target {
                right -= 1;
                continue;
            }
            if sum < target {
                left += 1;
                continue;
            }
            return vec![(left + 1) as i32, (right + 1) as i32];
        }
        return vec![];
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        // find length
        let mut length = 0;
        let mut h_ref = head.as_ref();
        while h_ref.is_some() {
            length += 1;
            let h_node = h_ref.unwrap();
            h_ref = h_node.next.as_ref();
        }
        if length <= 2 {
            return;
        }
        // move to mid
        let mut i = 1;
        let mut h_mut = head.as_mut();
        while i < length / 2 {
            i += 1;
            let h_node = h_mut.unwrap();
            h_mut = h_node.next.as_mut();
        }
        // mid_node.next = None, reverse [mid, length) node
        let mut h_node = h_mut.unwrap();
        let mut r_head = h_node.next.take();
        h_node.next = None;
        let mut r_last: Option<Box<ListNode>> = None;
        while r_head.as_ref().unwrap().next.is_some() {
            let mut left_node = r_head.unwrap();
            r_head = left_node.next;
            left_node.next = r_last;
            r_last = Some(left_node);
        }
        let mut r_node = r_head.as_mut().unwrap();
        r_node.next = r_last;
        println!("head {:?}, r_head: {:?}", head, r_head);

        // merge
        if let Some(node) = head {
            node.next = Self::merge(r_head, node.next.take());
        }
    }

    pub fn merge(
        mut left: Option<Box<ListNode>>,
        right: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (left.as_mut(), right.as_ref()) {
            (None, None) => None,
            (Some(_), None) => left,
            (None, Some(_)) => right,
            (Some(l), Some(_)) => {
                l.next = Self::merge(right, l.next.take());
                left
            }
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let mut old_node = head.as_mut();
        let mut val_status = (old_node.as_ref().unwrap().val, 1);
        let mut new_head = Some(Box::new(ListNode::new(0)));
        let mut new_node = new_head.as_mut();
        while old_node.as_ref().unwrap().next.is_some() {
            old_node = old_node.unwrap().next.as_mut();
            let old_ref = old_node.as_ref().unwrap();
            if old_ref.val == val_status.0 {
                val_status.1 += 1;
                continue;
            }
            // not same, check count
            if val_status.1 == 1 {
                let node = Some(Box::new(ListNode::new(val_status.0)));
                let node_node = new_node.unwrap();
                node_node.next = node;
                new_node = node_node.next.as_mut();
            }
            val_status.0 = old_ref.val;
            val_status.1 = 1;
        }
        if val_status.1 == 1 {
            let node = Some(Box::new(ListNode::new(val_status.0)));
            let node_node = new_node.unwrap();
            node_node.next = node;
        }
        return new_head.unwrap().next;
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut checked, mut pos) = (1 as usize, 1 as usize);
        let mut status = (nums[0], 1i32);
        while pos < nums.len() {
            if status.0 == nums[pos] {
                status.1 += 1;
                if status.1 > 2 {
                    pos += 1;
                    continue;
                }
                nums.swap(checked, pos);
                checked += 1;
                pos += 1;
                continue;
            }
            status.0 = nums[pos];
            status.1 = 1;
            nums.swap(checked, pos);
            checked += 1;
            pos += 1;
        }
        return checked as i32;
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut k = k;
        if head.is_none() {
            return head;
        }
        // get list length
        let mut length = 0;
        let mut node = head.as_mut();
        while node.is_some() {
            length += 1;
            let node_node = node.as_ref().unwrap();
            if node_node.next.is_none() {
                break;
            }
            node = node.unwrap().next.as_mut();
        }
        if length <= k {
            k = k % length;
        }
        if k == 0 {
            return head;
        }
        node = head.as_mut();
        let mut i = 1;
        while i < length - k {
            node = node.unwrap().next.as_mut();
            i += 1;
        }
        let mut node_node = node.unwrap();
        let mut new_head = node_node.next.clone();
        node_node.next = None;
        let mut new = new_head.as_mut();
        loop {
            let new_node = new.as_ref().unwrap();
            if new_node.next.is_none() {
                break;
            }
            new = new.unwrap().next.as_mut();
        }
        let new_node = new.unwrap();
        new_node.next = head;

        new_head
    }
}

//impl Solution {
//    #[allow(dead_code)]
//    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
//        //let mut pos = 0 as usize;
//        //let mut k = k as usize;
//        //while pos < nums.len(){
//        //    let new_pos = (pos + k) % nums.len();
//        //}
//        return;
//    }
//}

impl Solution {
    #[allow(dead_code)]
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let ver1: Vec<&str> = version1.split(".").collect();
        let ver2: Vec<&str> = version2.split(".").collect();
        let (mut iter1, mut iter2) = (ver1.iter(), ver2.iter());
        let (mut v1, mut v2) = (iter1.next(), iter2.next());
        loop {
            if v1 == None || v2 == None {
                break;
            }
            let str1 = v1.unwrap();
            let str2 = v2.unwrap();
            let code1 = str1.parse::<i32>().unwrap();
            let code2 = str2.parse::<i32>().unwrap();
            if code1 < code2 {
                return -1;
            }
            if code1 > code2 {
                return 1;
            }
            v1 = iter1.next();
            v2 = iter2.next();
        }
        if v1 == None {
            loop {
                if v2 == None {
                    break;
                }
                let str2 = v2.unwrap();
                let code2 = str2.parse::<i32>().unwrap();
                if code2 != 0 {
                    return -1;
                }
                v2 = iter2.next();
            }
            return 0;
        }
        if v2 == None {
            loop {
                if v1 == None {
                    break;
                }
                let str1 = v1.unwrap();
                let code1 = str1.parse::<i32>().unwrap();
                if code1 != 0 {
                    return 1;
                }
                v1 = iter1.next();
            }
            return 0;
        }
        return 0;
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort();
        nums2.sort();
        let mut intersec: Vec<i32> = vec![];
        let (mut i1, mut i2) = (0 as usize, 0 as usize);
        while (i1 < nums1.len()) && (i2 < nums2.len()) {
            if nums1[i1] < nums2[i2] {
                i1 += 1;
                continue;
            }
            if nums1[i1] > nums2[i2] {
                i2 += 1;
                continue;
            }
            intersec.push(nums1[i1]);
            i1 += 1;
            i2 += 1;
        }
        intersec
    }
}
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut head = head.clone();
        if head.is_none() {
            return true;
        }
        let (mut fast, mut slow) = (head.clone(), head.clone());
        // seek middle
        while fast.is_some() {
            slow = slow.unwrap().next;
            fast = fast.unwrap().next;
            if !fast.is_some() {
                break;
            }
            fast = fast.unwrap().next;
        }
        // now slow is in the middle, reverse [middle, none]
        let mut last: Option<Box<ListNode>> = None;
        while slow.is_some() {
            let mut slow_node = slow.unwrap();
            let tmp_next = slow_node.next;
            slow_node.next = last;
            last = Some(slow_node);
            slow = tmp_next;
        }
        slow = last;
        // now slow become last half list's head, compare
        // maybe slow has one more node
        while slow.is_some() && head.is_some() {
            let slow_node = slow.unwrap();
            let head_node = head.unwrap();
            if slow_node.val != head_node.val {
                return false;
            }
            slow = slow_node.next;
            head = head_node.next;
        }
        return true;
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut pair_num: i32 = 0;
        let mut mut_nums: Vec<i32> = nums.clone();
        mut_nums.sort();
        let mut right = mut_nums.len() - 1;
        while right > 0 {
            let mut left = 0;
            if mut_nums[right] - mut_nums[left] < k {
                break;
            }
            while left < right {
                if (mut_nums[right] - mut_nums[left]) > k {
                    while left < right && mut_nums[left] == mut_nums[left + 1] {
                        left += 1;
                    }
                    left += 1;
                    continue;
                }
                if mut_nums[right] - mut_nums[left] < k {
                    break;
                }
                pair_num += 1;
                break;
            }
            while right > 0 && mut_nums[right] == mut_nums[right - 1] {
                right -= 1;
                continue;
            }
            if right == 0 {
                continue;
            }
            right -= 1;
        }
        return pair_num;
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (s_b, t_b) = (s.as_bytes(), t.as_bytes());
        let (mut s_match, mut t_i) = (0 as usize, 0 as usize);
        while t_i < t_b.len() {
            if s_match == s_b.len() {
                break;
            }
            if s_b[s_match] == t_b[t_i] {
                s_match += 1;
            }
            t_i += 1;
        }
        if s_match == s_b.len() {
            return true;
        }
        false
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }
        let hay_bytes = haystack.as_bytes();
        let needle_bytes = needle.as_bytes();
        let (mut hy_p, mut match_p, mut match_b) = (0 as usize, 0 as usize, -1 as i32);
        while hy_p < hay_bytes.len() {
            if match_p == needle_bytes.len() {
                return match_b as i32;
            }
            if hay_bytes[hy_p] == needle_bytes[match_p] {
                if match_p == 0 {
                    match_b = hy_p as i32;
                }
                hy_p += 1;
                match_p += 1;
                continue;
            }
            // not same
            if match_p != 0 {
                hy_p = (match_b + 1) as usize;
                match_p = 0;
                match_b = -1;
                continue;
            }
            hy_p += 1;
        }
        return if match_p == needle_bytes.len() {
            match_b as i32
        } else {
            -1
        };
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let (mut ab_min, mut ab_max) = (0 as i32, 0 as i32);
        // find ab min
        let mut flag: bool = false;
        let mut i = 1 as usize;
        while i < nums.len() {
            if (nums[i - 1] > nums[i]) && (!flag) {
                flag = true;
                ab_min = nums[i];
            }
            if flag {
                if nums[i] < ab_min {
                    ab_min = nums[i];
                }
            }
            i += 1;
        }
        if !flag {
            return 0;
        }
        i = nums.len() - 2;
        flag = false;
        #[allow(unused_comparisons)]
        while i >= 0 {
            if (nums[i + 1] < nums[i]) && (!flag) {
                flag = true;
                ab_max = nums[i];
            }
            if flag {
                if nums[i] > ab_max {
                    ab_max = nums[i];
                }
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        // find the begin of ab_min
        let (mut l, mut r) = (0 as usize, nums.len() - 1);
        i = 0;
        while i < nums.len() {
            if nums[i] > ab_min {
                l = i;
                break;
            }
            i += 1;
        }
        i = nums.len() - 1;
        while i > 0 {
            if nums[i] < ab_max {
                r = i;
                break;
            }
            i -= 1;
        }
        if i == 0 {
            r = 0;
        }
        return if l >= r { 0 } else { (r - l + 1) as i32 };
    }
}

#[allow(dead_code)]
struct MedianFinder {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    #[allow(dead_code)]
    fn new() -> Self {
        MedianFinder { nums: Vec::new() }
    }

    #[allow(dead_code)]
    fn add_num(&mut self, num: i32) {
        self.nums.push(num);
    }

    #[allow(dead_code)]
    fn find_median(&mut self) -> f64 {
        self.nums.sort();
        let num_len = self.nums.len();
        if num_len % 2 == 0 {
            let lower = num_len / 2;
            let upper = lower / 2 + 1;
            let mid = (self.nums[lower] + self.nums[upper]) as f64 / (2 as f64);
            return mid;
        }
        let mid = num_len / 2;
        return self.nums[mid] as f64;
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut fast, mut slow) = (nums[0] as usize, nums[0] as usize);
        fast = nums[fast] as usize;
        fast = nums[fast] as usize;
        slow = nums[slow] as usize;
        while fast != slow {
            fast = nums[fast] as usize;
            fast = nums[fast] as usize;
            slow = nums[slow] as usize;
        }
        slow = nums[0] as usize;
        while fast != slow {
            fast = nums[fast] as usize;
            slow = nums[slow] as usize;
        }
        return fast as i32;
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut water: i32 = 0;
        let (mut left, mut right) = (0 as usize, height.len() - 1);
        let (mut left_max, mut right_max) = (0 as i32, 0 as i32);

        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    water += left_max - height[left];
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    water += right_max - height[right];
                }
                right -= 1;
            }
        }
        return water;
    }
}

//impl Solution {
//    #[allow(dead_code)]
//    pub fn is_palindrome(s: String) -> bool {
//        let mut letters = s.chars().filter(|x| x.is_alphanumeric());
//        while let (Some(c1), Some(c2)) = (letters.next(), letters.next_back()) {
//            if !(c1.eq_ignore_ascii_case(&c2)) {
//                return false;
//            }
//        }
//        true
//    }
//}

impl Solution {
    #[allow(dead_code)]
    pub fn is_happy(_n: i32) -> bool {
        true
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut left: usize = 0;
        let mut right: usize = s.len() - 1;
        while left < right {
            let tmp = s[left];
            s[left] = s[right];
            s[right] = tmp;
            left += 1;
            right -= 1;
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_volwel(s: String) -> String {
        let mut arr: Vec<char> = s.chars().collect();
        let (mut i, mut j) = (0, arr.len() - 1);
        fn is_vowel(s: char) -> bool {
            match s {
                'a' | 'e' | 'i' | 'o' | 'u' => return true,
                'A' | 'E' | 'I' | 'O' | 'U' => return true,
                _ => return false,
            }
        }
        while i < j {
            while i < j {
                if !is_vowel(arr[i]) {
                    i += 1;
                }
                break;
            }
            while i < j && !is_vowel(arr[j]) {
                j -= 1;
            }
            if i >= j {
                break;
            }
            arr.swap(i, j);
            i += 1;
            j -= 1;
        }
        arr.iter().collect()
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set_1: std::collections::HashSet<_> = nums1.iter().cloned().collect();
        let set_2: std::collections::HashSet<_> = nums2.iter().cloned().collect();
        let mut output: Vec<i32> = Vec::new();
        for &x in set_1.intersection(&set_2) {
            output.push(x);
        }
        output
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_words(s: String) -> String {
        let mut bytes = s.into_bytes();
        fn reverse(sub: &mut [u8], begin: usize, end: usize) {
            let mut begin = begin;
            let mut end = end;
            while begin < end {
                let tmp = sub[end];
                sub[end] = sub[begin];
                sub[begin] = tmp;
                begin += 1;
                end -= 1;
            }
        }
        let (mut begin, mut end) = (0, 0);
        while end < bytes.len() {
            // find space, get one word of bytes[begin, end]
            if bytes[end] == b' ' {
                reverse(&mut bytes, begin, end - 1);
                // move to next words, whitespace cannot be the end of words
                begin = end + 1;
            }
            end += 1;
        }
        if end > begin {
            reverse(&mut bytes, begin, end - 1);
        }
        match std::str::from_utf8(&bytes) {
            Ok(v) => return v.to_string(),
            Err(_e) => return "".to_string(),
        };
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut last_len: i32 = 0;
        let mut last_char: char = chars[0];
        let mut comp_size: i32 = 0;
        let mut comp_char: Vec<char> = Vec::new();

        fn digit_len(digit: i32) -> (i32, Vec<char>) {
            let mut d_len: i32 = 2;
            let mut digit_ = digit;
            let mut digit_vec: Vec<char> = Vec::new();
            if digit_ == 0 {
                return (0, digit_vec);
            }
            if digit_ == 1 {
                return (1, digit_vec);
            }
            let les_10 = digit_ % 10;
            digit_ = digit_ - les_10;
            while digit_ >= 10 {
                d_len += 1;
                digit_ = digit_ / 10;
            }
            let d_str = digit.to_string();
            digit_vec = Vec::from_iter(d_str.chars());
            return (d_len, digit_vec);
        }

        let mut i: usize = 0;
        while i < chars.len() {
            let char_ = chars[i];
            if char_ != last_char {
                let (digit_len, digit_char) = digit_len(last_len);
                if digit_len > 0 {
                    comp_char.push(last_char);
                    comp_char.extend(digit_char);
                }
                comp_size += digit_len;
                last_char = char_;
                last_len = 1;
                i += 1;
                continue;
            }
            last_len += 1;
            i += 1;
        }
        let (digit_len, digit_char) = digit_len(last_len);
        if digit_len > 0 {
            comp_char.push(last_char);
            comp_char.extend(digit_char);
        }
        comp_size += digit_len;
        let mut i: usize = 0;
        while i < comp_char.len() {
            chars[i] = comp_char[i];
            i += 1;
        }
        return comp_size;
    }
}

/* 0876 middle of the linked list */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = match &head {
            Some(node) => node,
            None => return None,
        };
        let mut slow = match &head {
            Some(node) => node,
            None => return None,
        };
        'seek_loop: loop {
            // slow move 1 step each time
            // fast move 2 step each time
            slow = match &slow.next {
                Some(node) => &node,
                None => break 'seek_loop,
            };
            let mut step = 2;
            while step >= 1 {
                fast = match &fast.next {
                    Some(node) => &node,
                    None => break 'seek_loop,
                };
                step -= 1;
            }
            if !fast.next.is_some() {
                break 'seek_loop;
            }
        }
        return Some(slow.clone());
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        // sort
        let mut people = people.clone();
        people.sort();
        // become two sum question
        let (mut left, mut right) = (0, people.len() - 1);
        let mut boat: i32 = 0;
        while left < right {
            if people[left] + people[right] > limit {
                boat += 1;
                right -= 1;
                continue;
            }
            left += 1;
            right -= 1;
            boat += 1;
        }
        // pick left people
        if left == right {
            boat += 1;
        }
        return boat;
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn valid_palindrome(s: String) -> bool {
        let s_bytes = s.as_bytes();
        if s.len() <= 2 {
            return true;
        }
        let (mut left, mut right) = (0 as usize, s.len() - 1);
        fn palindrome(s_bytes: &[u8], l: usize, r: usize) -> bool {
            let (mut left, mut right) = (l, r);
            while left < right {
                if s_bytes[left] == s_bytes[right] {
                    left += 1;
                    right -= 1;
                    continue;
                }
                return false;
            }
            return true;
        }
        while left < right {
            if s_bytes[left] == s_bytes[right] {
                left += 1;
                right -= 1;
                continue;
            }
            // check gap is left
            let pali = palindrome(s_bytes, left + 1, right);
            if pali {
                return true;
            }
            let pali = palindrome(s_bytes, left, right - 1);
            if pali {
                return true;
            }
            return false;
        }
        return true;
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= (1 as usize) {
            return;
        }
        let (mut red_end, mut blue_begin) = (0 as usize, nums.len() - 1);
        let mut i: usize = 0;
        while i < nums.len() && i <= blue_begin {
            // move red
            if nums[i] == 0 {
                nums.swap(i, red_end);
                // red is always handled
                red_end += 1;
                i += 1;
                continue;
            }
            // keep white in same place
            // move blue
            if nums[i] == 2 {
                nums.swap(i, blue_begin);
                // after blue swap, blue_begin may not handle
                if blue_begin == 0 {
                    return;
                }
                blue_begin -= 1;
                // check swaped i again
                continue;
            }
            i += 1;
        }
    }
}
