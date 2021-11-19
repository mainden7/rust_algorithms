struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i64> {
        if row_index == 0 {
            return vec![1];
        }
        if row_index == 1 {
            return vec![1, 1];
        }
        let mut row: Vec<i64> = vec![1, row_index as i64];
        for i in 1..row_index / 2 {
            let a = row[i as usize] * (row_index as i64 - i as i64);
            let b = a / (i as i64 + 1);
            row.push(b);
        }
        match row_index % 2 {
            0 => {
                // even row
                let mut r = row[0..(row.len() - 1)].to_vec();
                r.reverse();
                row.append(&mut r);
            }
            _ => {
                let mut r = row.clone();
                r.reverse();
                row.append(&mut r);
            }
        }
        row
    }
}

pub fn main() {
    let ans = Solution::get_row(30);
    println!("{:?}", ans);
}
