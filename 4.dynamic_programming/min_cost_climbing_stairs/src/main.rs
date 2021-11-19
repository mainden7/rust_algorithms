struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut row = vec![1000;cost.len()];
        row[0] = cost[0];
        row[1] = cost[1];
        for i in 2..row.len() {
            row[i] = (row[i - 1] + cost[i]).min(row[i - 2] + cost[i]);
        }
        println!("{:?}", row);
        row[row.len() - 1].min(row[row.len() - 2])
    }
}

fn main() {
    let ans = Solution::min_cost_climbing_stairs(
        // vec![1,100,1,1,1,100,1,1,100,1]
        vec![10, 15, 20]
    );
    println!("{:?}", ans);
}
