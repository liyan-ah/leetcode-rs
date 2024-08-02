use std::fmt::format;

struct Solution {}

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n == 0 {
            return false;
        }
        let mut left = n;
        loop {
            if left == 1 {
                return true;
            }
            if left % 2 != 0 && left % 3 != 0 && left % 5 != 0 {
                return false;
            }
            if left % 2 == 0 {
                left = left / 2;
            }
            if left % 3 == 0 {
                left = left / 3;
            }
            if left % 5 == 0 {
                left = left / 5;
            }
        }
    }
}

impl Solution {
    pub fn convert_to_base_n(num: i32, base: i32) -> String {
        let mut left = num;
        let mut v = vec![];
        loop {
            if left.abs() < base.abs() {
                break;
            }
            v.push(((left % base).abs() as u8 + '0' as u8) as char);
            left = left / base;
        }
        v.push((left.abs() as u8 + '0' as u8) as char);
        if num < 0 {
            v.push('-');
        }
        v.reverse();
        v.iter().collect()
    }
    pub fn convert_to_base7(num: i32) -> String {
        Self::convert_to_base_n(num, 7)
    }
}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let l = nums.len() as i32;
        let mut sum = ((1 + l) * l) / 2;
        for v in nums.iter() {
            sum -= *v;
        }
        sum
    }
}

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len()).for_each(|i| {
            let idx = (nums[i].abs() - 1) as usize;
            nums[idx] = -nums[idx].abs();
        });
        let mut ret = vec![];
        (0..nums.len()).for_each(|i| {
            if nums[i] > 0 {
                ret.push((i + 1) as i32);
            }
        });
        ret
    }
}

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut s = std::collections::HashSet::new();
        let mut last = None;
        loop {
            let mut round = None;
            for v in nums.iter() {
                let v = *v;
                if s.get(&v).is_some() {
                    continue;
                }
                match round {
                    Some(v1) => {
                        if v > v1 {
                            round = Some(v);
                        }
                    }
                    None => {
                        round = Some(v);
                    }
                }
            }
            if round.is_none() {
                return last.unwrap();
            } else if last.is_none() {
                last = round.clone();
            }
            if s.len() == 2 {
                return round.unwrap();
            }
            s.insert(round.unwrap());
        }
    }
}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let mut div_sum = 0;
        let mut div = 1;
        loop {
            if div * div > num {
                return div_sum == num;
            }
            if num % div == 0 {
                div_sum += div;
                div_sum += num / div;
            }
            div += 1;
        }
    }
}

impl Solution {
    pub fn sum_n(mut nums: Vec<i32>, val: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = vec![];

        let mut i = 0;
        while i < nums.len() {
            if i != 0 && nums[i] == nums[i - 1] {
                i += 1;
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            let target = val - nums[i];
            while left < right {
                if left != i + 1 && nums[left] == nums[left - 1] {
                    left += 1;
                    continue;
                }
                if right != nums.len() - 1 && nums[right] == nums[right + 1] {
                    right -= 1;
                    continue;
                }
                if nums[left] + nums[right] > target {
                    right -= 1;
                } else if nums[left] + nums[right] < target {
                    left += 1;
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                }
            }
        }

        res
    }
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        Solution::sum_n(nums, 0)
    }
}

impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let [.., x1, x2, x3] = nums[..] else { todo!() };
        if nums.len() == 3 {
            return x1 * x2 * x3;
        }
        if nums.len() > 3 {
            if nums[0] * nums[1] * x3 > x1 * x2 * x3 {
                return nums[0] * nums[1] * x3;
            }
        }

        x1 * x2 * x3
    }
}

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut str_1: Vec<char> = num1.chars().collect();
        let mut str_2: Vec<char> = num2.chars().collect();
        str_1.reverse();
        str_2.reverse();
        let mut sum = vec![];
        let mut i = 0;
        let mut up = 0;
        'add: loop {
            if i >= str_1.len() || i >= str_2.len() {
                break 'add;
            }
            let sum_v = str_1[i] as u8 + str_2[i] as u8 - '0' as u8 * 2 + up;
            sum.push((sum_v % 10 + '0' as u8) as char);
            up = sum_v / 10;
            i += 1;
        }
        while i < str_1.len() {
            if up > 0 {
                let sum_v = str_1[i] as u8 - '0' as u8 + up;
                sum.push((sum_v % 10 + '0' as u8) as char);
                up = sum_v / 10;
            } else {
                sum.push(str_1[i]);
            }
            i += 1;
        }
        while i < str_2.len() {
            if up > 0 {
                let sum_v = str_2[i] as u8 - '0' as u8 + up;
                sum.push((sum_v % 10 + '0' as u8) as char);
                up = sum_v / 10;
            } else {
                sum.push(str_2[i]);
            }
            i += 1;
        }
        if up > 0 {
            sum.push((up + '0' as u8) as char);
        }
        sum.reverse();

        sum.into_iter().collect()
    }
}

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for chunk in nums.chunks(2) {
            let [freq, val] = chunk else { todo!() };
            res.append(&mut vec![*val; freq.clone() as usize]);
        }
        res
    }
}

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        let mut sum = 0;
        'sum: loop {
            'once: loop {
                if num < 10 {
                    break 'once;
                }
                sum += num % 10;
                num = num / 10;
            }
            sum += num;
            if sum < 10 {
                break 'sum;
            }
            num = sum;
            sum = 0;
        }
        sum
    }
}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let (mut start, mut end) = (None, None);
        let mut res = vec![];
        for v in nums.iter() {
            match end {
                Some(e) => {
                    if *v == e + 1 {
                        end = Some(*v);
                    } else {
                        res.push(format!("{}->{}", start.unwrap(), e));
                        start = Some(*v);
                        end = None;
                    }
                }
                None => match start {
                    Some(s) => {
                        if *v == s + 1 {
                            end = Some(*v);
                        } else {
                            start = Some(*v);
                            res.push(format!("{s}"));
                        }
                    }
                    None => {
                        start = Some(*v);
                    }
                },
            }
        }
        match start {
            Some(s) => match end {
                Some(e) => {
                    res.push(format!("{s}->{e}"));
                }
                None => {
                    res.push(format!("{s}"));
                }
            },
            None => {}
        }

        res
    }
}

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let (mut s, mut l, mut i) = (-1, -1, 0);
        for idx in 0..nums.len() {
            if l < nums[idx] {
                i = idx;
                s = l;
                l = nums[idx];
                continue;
            }
            if s < nums[idx] {
                s = nums[idx];
            }
        }
        return if l >= 2 * s { i as i32 } else { -1 };
    }
}

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return true;
        }
        let (mut before, mut sub) = (nums[1], nums[1] - nums[0]);
        for i in 2..nums.len() {
            if sub * (nums[i] - before) <= 0 {
                return false;
            }
            if nums[i] - before != 0 {
                sub = nums[i] - before;
            }
            before = nums[i];
        }
        true
    }
}

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        if matrix.len() <= 1 {
            return true;
        }
        // check left
        for m in 0..matrix.len() {
            let raw = matrix[m][0];
            let mut l = 0;
            loop {
                if m + l >= matrix.len() {
                    break;
                }
                if l >= matrix[0].len() {
                    break;
                }
                if matrix[m + l][l] != raw {
                    return false;
                }
                l += 1;
            }
        }

        // check right
        for n in 1..matrix[0].len() {
            let raw = matrix[0][n];
            let mut r = 0;
            loop {
                if r >= matrix.len() {
                    break;
                }
                if n + r >= matrix[0].len() {
                    break;
                }
                if matrix[r][n + r] != raw {
                    return false;
                }
                r += 1;
            }
        }

        true
    }
}

impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let (x, y, z) = (&points[0], &points[1], &points[2]);
        // check if no k
        if y[0] == x[0] && y[0] == z[0] {
            return false;
        }
        // check if same point
        if (x[0] == y[0] && x[1] == y[1])
            || (x[0] == z[0] && x[1] == z[1])
            || (y[0] == z[0] && y[1] == z[1])
        {
            return false;
        }
        // check if same k
        let k_0 = (y[1] - x[1]) as f32 / (y[0] - x[0]) as f32;
        let k_1 = (z[1] - y[1]) as f32 / (z[0] - y[0]) as f32;
        let k_3 = (z[1] - x[1]) as f32 / (z[0] - x[0]) as f32;
        if k_0 == k_1 && k_0 == k_3 {
            false
        } else {
            true
        }
    }
}

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let mut inc = 0;
        let mut dec = 0;
        for i in 1..arr.len() {
            if arr[i] == arr[i - 1] {
                return false;
            }
            if arr[i] > arr[i - 1] && dec > 0 {
                return false;
            }
            if arr[i] < arr[i - 1] && inc == 0 {
                return false;
            }
            if arr[i] > arr[i - 1] {
                inc += 1;
            } else {
                dec += 1;
            }
        }
        if inc == 0 || dec == 0 {
            return false;
        }
        true
    }
}

impl Solution {
    pub fn self_divided(num: i32) -> bool {
        let mut left = num;
        loop {
            if left < 10 {
                break;
            }
            let v = left % 10;
            if v == 0 || num % v != 0 {
                return false;
            }
            left = left / 10;
        }
        if left == 0 || num % left == 0 {
            true
        } else {
            false
        }
    }

    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res = vec![];
        for v in left..right + 1 {
            if Self::self_divided(v) {
                res.push(v);
            }
        }
        res
    }
}

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let mut res = vec![];
        let mut min = arr[1] - arr[0];
        res.push(vec![arr[0], arr[1]]);
        for i in 2..arr.len() {
            if arr[i] - arr[i - 1] == min {
                res.push(vec![arr[i - 1], arr[i]]);
                continue;
            }
            if arr[i] - arr[i - 1] < min {
                res.clear();
                res.push(vec![arr[i - 1], arr[i]]);
                min = arr[i] - arr[i - 1];
            }
        }

        res
    }
}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut round = 0;
        for num in nums.iter() {
            if *num == 1 {
                round += 1;
                max = if round > max { round } else { max };
            } else {
                round = 0;
            }
        }

        max
    }
}

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for i in 0..matrix[0].len() {
            let mut row = vec![];
            for j in 0..matrix.len() {
                row.push(matrix[j][i]);
            }
            res.push(row);
        }
        res
    }
}

impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        num.reverse();
        let mut addition = 0;
        let mut i = 0;
        while k > 0 || addition > 0 {
            let v = k % 10;
            k = k / 10;
            let mut new_v = if i >= num.len() { 0 } else { num[i] };
            let sum = new_v + v + addition;
            new_v = sum % 10;
            addition = sum / 10;

            if i >= num.len() {
                num.push(new_v);
            } else {
                num[i] = new_v;
            }

            i += 1;
        }

        num.reverse();
        num
    }
}

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let mut cur_len = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                cur_len += 1;
                continue;
            }
            max_len = if cur_len > max_len { cur_len } else { max_len };
            cur_len = 1;
        }
        max_len = if cur_len > max_len { cur_len } else { max_len };
        max_len
    }
}

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        if flowerbed.len() <= 2 && n > 0 {
            return false;
        }
        for i in 1..flowerbed.len() {
            if n == 0 {
                return true;
            }
            if flowerbed[i - 1] == 1 {
                continue;
            }
            if i == flowerbed.len() - 1 || (i < (flowerbed.len() - 1) && flowerbed[i + 1] == 0) {
                n -= 1;
                flowerbed[i] = 1;
            }
        }
        return if n == 0 { true } else { false };
    }
}

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        if n == 1 {
            return true;
        }
        let mut value = std::collections::HashSet::new();
        value.insert(n);
        let mut round = 1000;
        loop {
            if round == 0 {
                return false;
            }
            let mut sum = 0;
            let mut contains_1 = false;
            while n > 10 {
                let and = n % 10;
                n = n / 10;
                sum += and * and;
                if and == 1 {
                    contains_1 = true;
                }
            }
            if sum == 1 {
                return true;
            }
            if value.get(&sum).is_some() {
                return if contains_1 { false } else { true };
            }
            n = sum;
            round -= 1;
        }
    }
}

impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        if n < 1 {
            return false;
        }
        while n > 1 {
            if n % 3 != 0 {
                return false;
            }
            n = n / 3;
        }

        return if n == 1 { true } else { false };
    }
}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return -1;
        }
        let mut left_sum = 0;
        for i in 0..nums.len() {
            left_sum += nums[i];
            if sum / 2 == left_sum {
                return i as _;
            }
        }
        -1
    }
}

impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        while n > 1 {
            if n % 2 != 0 {
                return false;
            }
            n = n / 2;
        }
        return if n == 1 { true } else { false };
    }
}

impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort();
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                return true;
            }
        }
        return false;
    }
}

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut l = area;
        let mut w = 1;
        let mut res = vec![l, w];
        while l * l >= area {
            if area % l == 0 {
                w = area / l;
                if res[0] - res[1] > l - w {
                    res = vec![l, w];
                }
            }
            l -= 1;
        }
        res
    }
}

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let mut sum = 0;
        for v in arr.iter() {
            sum += *v;
        }
        if sum % 3 != 0 {
            return false;
        }

        let mut sum_0 = 0;
        for i in 0..(arr.len() - 2) {
            sum_0 += arr[i];
            if sum_0 == sum / 3 {
                let mut sum_1 = 0;
                for j in i + 1..(arr.len() - 1) {
                    sum_1 += arr[j];
                    if sum_1 == sum / 3 {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl Solution {
    pub fn find_difference(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<Vec<i32>> {
        nums1.sort();
        nums2.sort();
        let mut answer_0 = vec![];
        let mut answer_1 = vec![];
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            if i >= 1 && nums1[i] == nums1[i - 1] {
                i += 1;
                continue;
            }
            if j >= 1 && nums2[j] == nums2[j - 1] {
                j += 1;
                continue;
            }
            if nums1[i] == nums2[j] {
                i += 1;
                j += 1;
                continue;
            }
            if nums1[i] < nums2[j] {
                answer_0.push(nums1[i]);
                i += 1;
                continue;
            }
            if nums1[i] > nums2[j] {
                answer_1.push(nums2[j]);
                j += 1;
                continue;
            }
        }
        while i < nums1.len() {
            if i >= 1 && nums1[i] == nums1[i - 1] {
                i += 1;
                continue;
            }
            answer_0.push(nums1[i]);
            i += 1;
        }
        while j < nums2.len() {
            if j >= 1 && nums2[j] == nums2[j - 1] {
                j += 1;
                continue;
            }
            answer_1.push(nums2[j]);
            j += 1;
        }

        vec![answer_0, answer_1]
    }
}

impl Solution {
    pub fn is_even(num: i32) -> bool {
        if num < 10 {
            return false;
        }
        if num < 100 {
            return true;
        }
        if num < 1000 {
            return false;
        }
        if num < 10000 {
            return true;
        }
        if num < 100000 {
            return false;
        }
        true
    }

    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for num in nums.iter() {
            if Self::is_even(*num) {
                count += 1;
            }
        }
        count
    }
}

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut new = vec![];
        let mut i = 0 as usize;
        while i < n as usize {
            new.push(nums[i]);
            new.push(nums[i + n as usize]);
            i += 1;
        }
        new
    }
}

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let num = (high - low + 1);
        if num % 2 == 0 {
            return num / 2;
        }
        return if low % 2 == 0 { num / 2 } else { num / 2 + 1 };
    }
}
