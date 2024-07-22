impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 1;
        let mut points_sort = points.to_vec();
        points_sort.sort_by(|x, y| x[0].cmp(&y[0]));
        let mut arrow = &points_sort[0];
        let mut point = points_sort[0][1];
        for pos in 1..points_sort.len() {
            if points_sort[pos][0] > point {
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
