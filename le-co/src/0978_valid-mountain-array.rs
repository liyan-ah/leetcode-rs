impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let mut inc = 0;
        let mut dec = 0;
        for i in 1..arr.len() {
            if arr[i] == arr[i - 1] {
                return false;
            }
            if arr[i] > arr[i - 1] && dec > 0 {
                return false;
            }
            if arr[i] < arr[i - 1] && inc == 0 {
                return false;
            }
            if arr[i] > arr[i - 1] {
                inc += 1;
            } else {
                dec += 1;
            }
        }
        if inc == 0 || dec == 0 {
            return false;
        }
        true
    }
}
