struct Solution {}

impl Solution {
    pub fn min_distance(w1: String, w2: String) -> i32 {
        let n = w1.len();
        let m = w2.len();
        let mut table = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                let adj = if w1.chars().nth(i) == w2.chars().nth(j) {
                    1
                } else {
                    0
                };
                let arr = [table[i][j + 1], table[i + 1][j], table[i][j] + adj];
                let max = arr.iter().max();
                table[i + 1][j + 1] = *max.unwrap();
            }
        }

        println!("{:?}", table);

        (m + n - 2 * table[n][m]) as i32
    }
}

fn main() {
    let ans = Solution::min_distance("leetcode".to_string(), "etco".to_string());
    println!("{}", ans);
}
