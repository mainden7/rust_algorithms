struct Solution {}

fn explore_island(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
    if i > grid.len() - 1 || j > grid[0].len() - 1 || grid[i][j] != '1' {
        return;
    }
    grid[i][j] = '*';
    explore_island(grid, i + 1, j);
    explore_island(grid, i, j + 1);
    if i > 0 {
        explore_island(grid, i - 1, j);
    }
    if j > 0 {
        explore_island(grid, i, j - 1);
    }
}

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    count += 1;
                    explore_island(&mut grid, i, j);
                }
            }
        }

        count
    }
}

fn main() {
    let ans = Solution::num_islands(vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ]);
    println!("{:?}", ans);
}
