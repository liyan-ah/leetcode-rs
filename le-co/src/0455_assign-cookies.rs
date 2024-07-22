impl Solution {
    #[allow(dead_code)]
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort();
        s.sort();
        let mut content = 0;
        let (mut g_p, mut s_p) = (0 as usize, 0 as usize);
        while g_p < g.len() && s_p < s.len() {
            while s_p < s.len() {
                if g[g_p] <= s[s_p] {
                    content += 1;
                    break;
                }
                s_p += 1;
            }
            s_p += 1;
            g_p += 1;
        }

        content
    }
}
