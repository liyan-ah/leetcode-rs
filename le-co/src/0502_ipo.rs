impl Solution {
    #[allow(dead_code)]
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut w = w;
        let mut cap_pro: Vec<(&i32, &i32)> = capital.iter().zip(profits.iter()).collect();
        cap_pro.sort_by(|a, b| (a.0.cmp(&b.0)));
        let mut heap = std::collections::BinaryHeap::new();
        let mut j = 0 as usize;
        for i in 0..k {
            while j < cap_pro.len() && *cap_pro[j].0 <= w {
                heap.push(*cap_pro[j].1);
                j += 1;
            }
            let max_profit = heap.pop();
            if max_profit.is_some() {
                w += max_profit.unwrap();
            }
        }
        w
    }
}
