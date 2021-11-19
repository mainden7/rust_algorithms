struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut x1, mut x2, mut mask) = (0, 0, 0);
        for i in nums {
            x2 ^= x1 & i;
            x1 ^= i;
            mask = !(x1 & x2);
            x2 &= mask;
            x1 &= mask;
        }
        x1
    }
}

pub fn main() {
    println!("{}", Solution::single_number(vec![1, 1, 2, 1, 2, 3, 2]));
}
