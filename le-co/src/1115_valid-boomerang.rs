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
