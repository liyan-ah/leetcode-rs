impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut a_d = 0;
        let mut cons_l = 0;
        for c in s.chars().into_iter() {
            match c {
                'A' => {
                    a_d += 1;
                    if a_d >= 2 {
                        return false;
                    }
                    cons_l = 0;
                }
                'L' => {
                    cons_l += 1;
                    if cons_l >= 3 {
                        return false;
                    }
                }
                'P' => {
                    cons_l = 0;
                }
                _ => {
                    cons_l = 0;
                }
            }
        }
        true
    }
}
