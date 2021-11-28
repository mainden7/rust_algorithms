fn explore_province(grid: &mut Vec<Vec<i32>>, row: usize) {
    for i in 0..grid.len() {
        if grid[row][i] == 1 {
            grid[row][i] = -1;
            grid[i][row] = -1;
            explore_province(grid, i);
        }
    }
}

fn count_provinces(mut is_connected: Vec<Vec<i32>>) -> i32 {
    if is_connected.len() == 1 {
        return 1;
    }
    let n = is_connected.len();
    let mut provinces = 0;
    for i in 0..n {
        for j in 0..n {
            if is_connected[i][j] == 1 {
                provinces += 1;
                explore_province(&mut is_connected, i);
            }
        }
    }
    provinces
}

fn main() {
    let ans = count_provinces(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]);
    assert_eq!(ans, 2);
    let ans = count_provinces(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
    assert_eq!(ans, 3);
}
