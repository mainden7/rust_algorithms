use std::collections::VecDeque;

pub fn max_area(mut grid: Vec<Vec<i32>>) -> i32 {
    let (n, m) = (grid.len(), grid[0].len());

    fn dfs(i: usize, j: usize, grid: &mut Vec<Vec<i32>>) -> i32 {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((i, j));
        let mut sq = 0;
        loop {
            match queue.pop_front() {
                None => break,
                Some((i, j)) => {
                    if grid[i][j] != 1 {
                        continue;
                    }

                    sq += 1;
                    grid[i][j] = -1;

                    if i > 0 && grid[i - 1][j] == 1 {
                        queue.push_back((i - 1, j));
                    }
                    if i < grid.len() - 1 && grid[i + 1][j] == 1 {
                        queue.push_back((i + 1, j));
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        queue.push_back((i, j - 1));
                    }
                    if j < grid[0].len() - 1 && grid[i][j + 1] == 1 {
                        queue.push_back((i, j + 1));
                    }
                }
            }
        }

        sq
    }
    let mut max_square = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                let sq = dfs(i, j, &mut grid);
                max_square = max_square.max(sq);
            }
        }
    }
    max_square
}

fn main() {
    let ans = max_area(vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ]);
    println!("{}", ans);
    assert_eq!(ans, 6);
}
