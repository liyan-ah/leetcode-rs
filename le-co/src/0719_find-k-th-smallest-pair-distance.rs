
impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let n: usize = nums.len();

        let (mut low, mut high) = (0 as i32, nums[n-1]-nums[0]);
        while low < high{
            let m:i32 = (low+high)/2;
            let mut count:usize=0;

            let mut i:usize = 0;
            for j in 0..n{
                while nums[j] - nums[i] > m{
                    i += 1;
                }
                count += j-i;
            }
            if count < k as usize{
                low = m+1;
            }else{
                high = m;
            }

        }

        low
    }
}
