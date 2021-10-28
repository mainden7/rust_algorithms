use std::primitive;


struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {

        fn back(open: i32, close: i32, stack: &mut Vec<String>, mut res: Vec<String>, n: i32) -> Vec<String> {
            if open == close && close == n {
                res.push(stack.join(""));                
            }
            if open < n {
                stack.push(String::from("("));
                res = back(open + 1, close, stack, res, n);
                stack.pop();
            }
            if close < open {
                stack.push(String::from(")"));
                res = back(open, close + 1, stack, res, n);
                stack.pop();
            }
            return res;

        }
        let mut stack: Vec<String> = vec![];
        back(0, 0, &mut stack, vec![], n)
        
    }
}

fn main() {
    let ans = Solution::generate_parenthesis(3);
    println!("{:?}", ans);
}
