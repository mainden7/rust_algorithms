struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = strs[0].clone();

        for str in &strs {
            if res == "" {
                break;
            }

            while !str.starts_with(&res) {
                res.pop();
            }
        }
        res
    }
}
fn main() {
    let ans = Solution::longest_common_prefix(
        vec!["dog", "flow", "flight"]
            .iter()
            .map(|&x| x.to_string())
            .collect(),
    );
}
