struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut row = vec![amount + 1; (amount + 1) as usize];
        let min_coin = *coins.iter().min().unwrap() as usize;
        for i in 0..min_coin {
            row[i] = 0;
        }
        for idx in min_coin..row.len() {
            for coin in coins.iter() {
                if idx as i32 - coin < 0 {
                    continue;
                } else {
                    row[idx] = row[idx].min(1 + row[idx - (*coin as usize)]);
                }
            }
        }
        if row[amount as usize] != amount + 1 {
            row[amount as usize]
        } else {
            -1
        }
    }
}
fn main() {
    Solution::coin_change(vec![1, 2, 5], 11);
}
