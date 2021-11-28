use std::collections::VecDeque;

fn explore(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) {
    let mut queue = VecDeque::new();
    queue.push_front((x, y));

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if x.min(y) < 0 || x > (grid[0].len() - 1) as i32 || y > (grid.len() - 1) as i32 {
            continue;
        }
        if grid[y as usize][x as usize] == 1 {
            grid[y as usize][x as usize] = -1;

            queue.push_back((x - 1, y));
            queue.push_back((x + 1, y));
            queue.push_back((x, y - 1));
            queue.push_back((x, y + 1));
        }
    }
}

fn enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    if m <= 2 || n <= 2 {
        return 0;
    }
    // run on all boundary cells and mark all adjucent cells
    let (mut x, mut y, mut dx, mut dy) = (0, 0, 1, 0);
    loop {
        // turnover when get to boundary corner
        if (x + dx == n as i32) || (y + dy == m as i32) || (x + dx < 0) {
            let tmp = dx;
            dx = -dy;
            dy = tmp;
        }

        if grid[y as usize][x as usize] == 1 {
            explore(&mut grid, x, y);
        }
        // finish when come back to first cell
        if x + dx == 0 && y + dy == 0 {
            break;
        }

        x += dx;
        y += dy;
    }

    let mut counter = 0;
    for i in 1..m {
        for j in 1..n {
            if grid[i][j] == 1 {
                counter += 1;
            }
        }
    }
    counter
}
fn main() {
    let ans = enclaves(vec![
        vec![0, 0, 0, 1, 1, 1, 0, 1, 0, 0],
        vec![1, 1, 0, 0, 0, 1, 0, 1, 1, 1],
        vec![0, 0, 0, 1, 1, 1, 0, 1, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 1, 0, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0, 0, 1, 0],
        vec![0, 0, 1, 0, 1, 1, 1, 1, 0, 1],
        vec![0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
        vec![0, 0, 1, 0, 0, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 1],
    ]);
    assert_eq!(ans, 3);
}
