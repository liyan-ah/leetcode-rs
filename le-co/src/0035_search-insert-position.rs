impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let (mut s, mut e, mut next) = (0, nums.len() - 1, 0);
        while s < e {
            if nums[s] >= target {
                return s as i32;
            }
            if nums[e] < target {
                return (e + 1) as i32;
            }
            if nums[e] == target {
                return e as i32;
            }
            next = (s + e) / 2;
            if nums[next] > target {
                e = next - 1;
            } else if nums[next] == target {
                return next as i32;
            } else {
                s = next + 1;
            }
        }

        if nums[s] >= target {
            return s as i32;
        }
        (e + 1) as i32
    }
}
