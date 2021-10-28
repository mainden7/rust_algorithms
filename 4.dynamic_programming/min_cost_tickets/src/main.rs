use std::{collections::HashSet, iter::FromIterator};

struct Solution {}

impl Solution {
    pub fn min_cost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        if days.len() < 2 {
            return *costs.iter().min().unwrap();
        }
        let days_set: HashSet<i32> = HashSet::from_iter(days);
        let max_day = days_set.iter().max().unwrap();
        let mut row = vec![0; (max_day + 1) as usize];
        for idx in 1..row.len() {
            if !days_set.contains(&(idx as i32)) {
                row[idx] = row[idx - 1];
            } else {
                if idx < 7 {
                    row[idx] = *[row[idx - 1] + costs[0], costs[1], costs[2]]
                        .iter()
                        .min()
                        .unwrap();
                } else if idx >= 6 && idx < 30 {
                    row[idx] = *[row[idx - 1] + costs[0], row[idx - 7] + costs[1], costs[2]]
                        .iter()
                        .min()
                        .unwrap()
                } else {
                    row[idx] = *[
                        costs[0] + row[idx - 1],
                        costs[1] + row[idx - 7],
                        costs[2] + row[idx - 30],
                    ]
                    .iter()
                    .min()
                    .unwrap();
                }
            }
        }
        return row[*max_day as usize];
    }
}
fn main() {
    let ans = Solution::min_cost_tickets(vec![1, 4, 6, 7, 8, 20], vec![7, 2, 15]);
    println!("{}", ans);
}
