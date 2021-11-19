struct Solution {}

impl Solution {
    pub fn let_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut is_cols: bool = false;
        let (m, n) = (matrix.len(), matrix[0].len());
        for i in 0..m {
            if matrix[i][0] == 0 {
                is_cols = true;
            }
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        if matrix[0][0] == 0 {
            for i in 0..n {
                matrix[0][i] = 0;
            }

        }
        if is_cols {
            for i in 0..m {
                matrix[i][0] = 0;

            }
        }
    }
}

fn main() {
    let mut x = vec![
        vec![1, 2, 3, 4],
        vec![5, 0, 7, 8],
        vec![0, 10, 11, 12],
        vec![13, 14, 15, 0],
    ];
    Solution::let_zeroes(&mut x);
}
