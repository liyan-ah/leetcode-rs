impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for i in 0..matrix[0].len() {
            let mut row = vec![];
            for j in 0..matrix.len() {
                row.push(matrix[j][i]);
            }
            res.push(row);
        }
        res
    }
}
