use std::{cmp::Ordering, collections::HashMap};
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
