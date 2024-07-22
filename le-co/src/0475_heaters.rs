impl Solution {
    #[allow(dead_code)]
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut houses = houses;
        let mut heaters = heaters;
        houses.sort();
        heaters.sort();
        let mut house = 0 as usize;
        let mut heater = 0 as usize;
        let mut radiu = 0 as i32;
        while house < houses.len() {
            let h_radiu = (houses[house] - heaters[heater]).abs();
            let mut h_cur = h_radiu;
            while heater < heaters.len() - 1 {
                let h_next = (houses[house] - heaters[heater + 1]).abs();
                if h_next <= h_cur {
                    h_cur = std::cmp::min(h_cur, h_next);
                    heater += 1;
                } else {
                    break;
                }
            }
            if h_cur > radiu {
                radiu = h_cur;
            }
            house += 1;
        }

        radiu
    }
}
