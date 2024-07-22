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
