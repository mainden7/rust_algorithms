use std::collections::{HashMap, VecDeque};
struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut pairs: HashMap<char, char> = HashMap::new();
        pairs.insert('(', ')');
        pairs.insert(')', '(');
        pairs.insert('[', ']');
        pairs.insert(']', '[');
        pairs.insert('{', '}');
        pairs.insert('}', '{');
        for p in s.chars() {
            if p == '(' || p == '[' || p == '{' {
                stack.push_back(p);
            } else {
                if stack.len() > 0 {
                    let pare = stack.pop_back().unwrap();
                    if p != pairs[&pare] {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }
}

pub fn main() {
    let ans = Solution::is_valid(String::from("()(())"));
    println!("{}", ans);
}
