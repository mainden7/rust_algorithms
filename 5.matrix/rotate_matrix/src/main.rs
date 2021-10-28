use std::mem::swap;

struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i..n {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        for i in 0..n {
            for j in 0..n / 2 {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[i][n - j - 1];
                matrix[i][n - j - 1] = tmp;
            }
        }

    }
}

fn main() {
    let mut mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let ans = Solution::rotate(&mut mat);
    println!("{:?}", ans);
}
