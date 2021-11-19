use std::collections::HashSet;

pub fn can_place(
    board: &mut Vec<Vec<i32>>,
    row: usize,
    col: usize,
    cols_set: &mut HashSet<usize>,
) -> bool {
    // check if we already have a Queen in a givent column
    if cols_set.contains(&col) {
        return false;
    }

    // check the diagonales
    for i in 0..row {
        let diff = row - i;
        if ((col >= diff) && board[i][col - diff] == 1)
            || (col + diff < board.len() && board[i][col + diff] == 1)
        {
            return false;
        }
    }

    true
}

pub fn bactrack(
    board: &mut Vec<Vec<i32>>,
    row: usize,
    to_place: usize,
    ans: &mut Vec<Vec<String>>,
    cols_set: &mut HashSet<usize>,
) {
    if to_place == 0 {
        let res = to_string(&board);
        ans.push(res);
        return;
    }

    for col in 0..board[0].len() {
        if can_place(board, row, col, cols_set) {
            board[row][col] = 1;
            cols_set.insert(col);
            bactrack(board, row + 1, to_place - 1, ans, cols_set);
            board[row][col] = 0;
            cols_set.remove(&col);
        }
    }
}
fn to_string(board: &Vec<Vec<i32>>) -> Vec<String> {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|&x| if x == 0 { "." } else { "Q" })
                .collect()
        })
        .collect()
}

pub fn solve(n: i32) -> Vec<Vec<String>> {
    if n == 1 {
        return vec![vec!["Q".to_string()]];
    }
    let n = n as usize;
    let mut board = vec![vec![0; n]; n];
    let mut ans: Vec<Vec<String>> = vec![];
    let mut cols_set: HashSet<usize> = HashSet::new();
    bactrack(&mut board, 0, n, &mut ans, &mut cols_set);
    ans
}

pub fn main() {
    let ans = solve(4);
    println!("{:?}", ans);
    assert_eq!(ans.len(), 2);
}
