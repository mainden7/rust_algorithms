struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut table: Vec<Vec<i32>> = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    table[i][j] = grid[i][j];
                    continue;
                }
                if i == 0 && j > 0 {
                    table[i][j] = table[i][j - 1] + grid[i][j];
                    continue;
                }
                if j == 0 && i > 0 {
                    table[i][j] = table[i - 1][j] + grid[i][j];
                    continue;
                }
                table[i][j] = (table[i - 1][j] + grid[i][j]).min(table[i][j - 1] + grid[i][j]);
            }
        }
        table[m - 1][n - 1]
    }
}

fn main() {
    let ans = Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]);
    println!("{:?}", ans);
}
