impl Solution {
    #[allow(dead_code)]
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut gas_sum = 0;
        let mut start_sum = 0;
        let mut start = 0;
        for i in 0..gas.len() {
            start_sum += gas[i] - cost[i];
            gas_sum += gas[i] - cost[i];
            if start_sum < 0 {
                start = ((i + 1) % gas.len()) as i32;
                start_sum = 0;
            }
        }
        if gas_sum >= 0 {
            return start;
        }
        return -1;
    }
}
