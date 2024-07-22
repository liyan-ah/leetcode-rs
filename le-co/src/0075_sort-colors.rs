impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= (1 as usize){
            return;
        }
        let (mut red_end, mut blue_begin) = (0 as usize, nums.len()-1);
        let mut i: usize = 0;
        while i < nums.len() && i <= blue_begin{
            // move red
            if nums[i] == 0{
                nums.swap(i, red_end);
                // red is always handled
                red_end += 1;
                i += 1;
                continue;
            }
            // keep white in same place
            // move blue
            if nums[i] == 2{
                nums.swap(i, blue_begin);
                // after blue swap, blue_begin may not handle
                if blue_begin == 0{
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
