impl Solution {
    #[allow(dead_code)]
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() == 1 {
            return 1;
        }
        let mut candy_num = 1;
        let mut status = (1, 0); // inc, dec
        let mut inc_max = 0;
        for pos in 1..ratings.len() {
            if ratings[pos] == ratings[pos - 1] {
                candy_num += 1;
                status.0 = 1;
                status.1 = 0;
                inc_max = status.0;
            } else if ratings[pos] > ratings[pos - 1] {
                status.0 += 1;
                candy_num += status.0;
                status.1 = 0;
                inc_max = status.0;
            } else if ratings[pos] < ratings[pos - 1] {
                status.0 = 1;
                status.1 += 1;
                if status.1 >= inc_max {
                    candy_num += 1;
                }
                candy_num += status.1;
            }
        }
        candy_num
    }
}
