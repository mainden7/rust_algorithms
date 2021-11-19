struct Solution {}

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut table = vec![vec![10001; n]; m];

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    table[i][j] = 0;
                } else {
                    if i > 0 {
                        table[i][j] = table[i][j].min(table[i - 1][j] + 1);
                    }
                    if j > 0 {
                        table[i][j] = table[i][j].min(table[i][j - 1] + 1);
                    }
                }
            }
        }
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if mat[i][j] == 0 {
                    table[i][j] = 0;
                } else {
                    if i < m - 1 {
                        table[i][j] = table[i][j].min(table[i + 1][j] + 1);
                    }
                    if j < n - 1 {
                        table[i][j] = table[i][j].min(table[i][j + 1] + 1);
                    }
                }
            }
        }
        table
    }
}

fn main() {
    let ans = Solution::update_matrix(vec![
        vec![0, 1, 0, 1, 1],
        vec![1, 1, 0, 0, 1],
        vec![0, 0, 0, 1, 0],
        vec![1, 0, 1, 1, 1],
        vec![1, 0, 0, 0, 1],
    ]);
    println!("{:?}", ans);
}
