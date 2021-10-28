struct Solution {}

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut x = 0;
        for op in operations.iter() {
            if op.as_str() == "--X" || op == "X--" {
                x -= 1;
            } else {
                x += 1;
            }
        }
        x
    }
}

fn main() {
    let ans = Solution::final_value_after_operations(vec!["--X", "X--"].iter().map(|&x| x.to_string()).collect());
    println!("{:?}", ans);
}