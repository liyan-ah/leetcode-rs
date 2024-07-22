impl Solution {
    #[allow(dead_code)]
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut new_list: Vec<Vec<i32>> = vec![];
        let (mut first, mut second) = (0 as usize, 0 as usize);
        while first < first_list.len() && second < second_list.len() {
            if first_list[first][0] > second_list[second][1] {
                second += 1;
                continue;
            }
            if first_list[first][1] < second_list[second][0] {
                first += 1;
                continue;
            }
            let v_s = std::cmp::max(first_list[first][0], second_list[second][0]);
            let v_e = std::cmp::min(first_list[first][1], second_list[second][1]);
            if second_list[second][1] > first_list[first][1] {
                first += 1;
            } else {
                second += 1;
            }
            new_list.push(vec![v_s, v_e]);
        }
        new_list
    }
}
