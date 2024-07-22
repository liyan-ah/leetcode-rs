impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let mut res = vec![];
        let mut min = arr[1] - arr[0];
        res.push(vec![arr[0], arr[1]]);
        for i in 2..arr.len() {
            if arr[i] - arr[i - 1] == min {
                res.push(vec![arr[i - 1], arr[i]]);
                continue;
            }
            if arr[i] - arr[i - 1] < min {
                res.clear();
                res.push(vec![arr[i - 1], arr[i]]);
                min = arr[i] - arr[i - 1];
            }
        }

        res
    }
}
