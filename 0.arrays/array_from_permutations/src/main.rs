struct Solution {}

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        // let mut res: Vec<i32> = Vec::new();
        // for i in 0..nums.len() {
        //     res.push(nums[nums[i] as usize]);
        // }
        // res
        nums.iter().map(|x| nums[*x as usize]).collect()
    }
}

fn main() {
    let ans = Solution::build_array(vec![0, 2, 1, 5, 3, 4]);
    println!("{:?}", ans);
}