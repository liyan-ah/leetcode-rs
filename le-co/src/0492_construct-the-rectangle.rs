impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut l = area;
        let mut w = 1;
        let mut res = vec![l, w];
        while (w * w) <= area {
            if area % w == 0 {
                l = area / w;
                if res[0] - res[1] > l - w {
                    res = vec![l, w];
                }
            }
            w += 1;
        }
        res
    }
}
