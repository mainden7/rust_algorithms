struct Solution {}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let bytes = format!("{:b}", num);
        let ones = bytes.matches("1").count();
        (ones - 1 + bytes.len()) as i32
    }
}

fn main() {
    let ans = Solution::number_of_steps(14);
    println!("{:?}", ans);
}
