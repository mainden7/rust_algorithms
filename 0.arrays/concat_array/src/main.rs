struct Solution {}

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len() * 2];
        for (idx, i) in nums.iter().enumerate() {
            res[idx] = *i;
            res[idx + nums.len()] = *i;
        }
        res
        // [nums.clone(), nums].concat()
    }
}

fn main() {
    let ans = Solution::get_concatenation(vec![1, 2, 3]);
    println!("{:?}", ans);
}