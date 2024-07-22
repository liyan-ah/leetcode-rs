impl Solution {
    #[allow(dead_code)]
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let (mut even, mut odd) = (0, 0);
        for p in 0..position.len() {
            let pos = position[p] % 2;
            match pos {
                0 => even += 1,
                1 => odd += 1,
                _ => even += 0,
            }
        }
        if even > odd {
            return odd;
        }
        even
    }
}
