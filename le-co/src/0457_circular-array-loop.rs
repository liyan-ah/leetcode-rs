impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return false;
        }
        fn next(nums: &Vec<i32>, pos: usize) -> usize {
            ((nums.len() as i32) + (pos as i32) + nums[pos]) as usize % nums.len()
        }
        let mut begin = 0 as usize;
        while begin < nums.len() {
            while  begin < nums.len() && begin == next(nums.as_ref(), begin) {
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
