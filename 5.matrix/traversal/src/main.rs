struct Solution {}

impl Solution {
    pub fn row_wise(matrix: &Vec<Vec<i32>>) {
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                println!("{}", matrix[i][j]);
            }
        }
    }

    pub fn col_wise(matrix: &Vec<Vec<i32>>) {
        for i in 0..matrix[0].len() {
            for j in 0..matrix.len() {
                println!("{}", matrix[j][i]);
            }
        }
    }

    pub fn spiral(matrix: &Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut x, mut y, mut dx, mut dy) = (0, 0, 1, 0);
        let mut visited: Vec<(i32, i32)> = vec![];
        let mut res: Vec<i32> = vec![];
        for _ in 0..m * n {
            if visited.contains(&(y + dy, x + dx))
                || !(0..n).contains(&((x + dx) as usize))
                || !(0..m).contains(&((y + dy) as usize))
            {
                let tmp = dx;
                dx = -dy;
                dy = tmp;
            }

            visited.push((y, x));
            res.push(matrix[y as usize][x as usize]);
            x = x + dx;
            y = y + dy;
        }
        res
    }
}

fn main() {
    let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    // Solution::row_wise(&mat);
    // Solution::col_wise(&mat);
    Solution::spiral(&mat);
}
