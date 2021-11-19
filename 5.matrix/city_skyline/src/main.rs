// https://leetcode.com/problems/max-increase-to-keep-city-skyline/
struct Solution {}

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut total = 0;
        let n = grid.len();

        let mut max_row = vec![0; n];
        let mut max_col = vec![0; n];

        for row in 0..(n as usize) {
            for col in 0..(n as usize) {
                max_row[row] = max_row[row].max(grid[row][col]);
                max_col[col] = max_col[col].max(grid[row][col]);
            }
        }

        for row in 0..(n as usize) {
            for col in 0..(n as usize) {
                let t = (max_row[row].min(max_col[col]) - grid[row][col]).abs();
                total += t
            }
        }
        total
    }
}
fn main() {
    let ans = Solution::max_increase_keeping_skyline(vec![
        vec![3, 0, 8, 4],
        vec![2, 4, 5, 7],
        vec![9, 2, 6, 3],
        vec![0, 3, 1, 0],
    ]);
    assert_eq!(ans, 35);
}
