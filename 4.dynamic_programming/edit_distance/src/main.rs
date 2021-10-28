struct Solution {}

impl Solution {
    pub fn edit_distance(word1: String, word2: String) -> i32 {
        let (m, n) = (word1.len(), word2.len());
        let mut table: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];

        for i in 0..m + 1 {
            table[i][0] = i as i32;
        }
        for i in 0..n + 1 {
            table[0][i] = i as i32;
        }

        let bw1 = word1.as_bytes();
        let bw2 = word2.as_bytes();

        for i in 1..(m + 1) {
            for j in 1..(n + 1) {
                if bw1[i - 1] == bw2[j - 1] {
                    table[i][j] = table[i - 1][j - 1];
                } else {
                    table[i][j] = 1 + [table[i - 1][j], table[i][j - 1], table[i - 1][j - 1]]
                        .iter()
                        .min()
                        .unwrap();
                }
            }
        }
        table[m][n]
    }
}
fn main() {
    let ans = Solution::edit_distance(String::from("horse"), String::from("horse"));
    println!("{}", ans);
}
