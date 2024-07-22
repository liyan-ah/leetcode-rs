impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        // change count for 5, 10, 20
        let mut change = (0, 0);
        fn find_change(change: &mut (i32, i32), need: i32) -> bool {
            if change.0 == 0 && change.1 == 0 {
                return false;
            }
            match need {
                // pay 20, need change 15
                15 => {
                    if change.1 > 0 && change.0 > 0 {
                        change.1 -= 1;
                        change.0 -= 1;
                    } else if change.0 >= 3 {
                        change.0 -= 3;
                    } else {
                        return false;
                    }
                }
                // pay 10, need change 5
                5 => {
                    if change.0 > 0 {
                        change.0 -= 1;
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
            true
        }

        fn set_change(change: &mut (i32, i32), pay: i32) -> bool {
            match pay {
                5 => change.0 += 1,
                10 => change.1 += 1,
                _ => return false,
            }
            true
        }
        for i in 0..bills.len() {
            if bills[i] > 5 {
                let ok = find_change(&mut change, bills[i] - 5);
                if !ok {
                    return false;
                }
            }
            set_change(&mut change, bills[i]);
        }
        true
    }
}
