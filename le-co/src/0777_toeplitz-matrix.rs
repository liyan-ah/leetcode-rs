impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        if matrix.len() <= 1 {
            return true;
        }
        // check left
        for m in 0..matrix.len() {
            let raw = matrix[m][0];
            let mut l = 0;
            loop {
                if m + l >= matrix.len() {
                    break;
                }
                if l >= matrix[0].len() {
                    break;
                }
                if matrix[m + l][l] != raw {
                    return false;
                }
                l += 1;
            }
        }

        // check right
        for n in 1..matrix[0].len() {
            let raw = matrix[0][n];
            let mut r = 0;
            loop {
                if r >= matrix.len() {
                    break;
                }
                if n + r >= matrix[0].len() {
                    break;
                }
                if matrix[r][n + r] != raw {
                    return false;
                }
                r += 1;
            }
        }

        true
    }
}
