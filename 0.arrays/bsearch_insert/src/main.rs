use core::num;

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut curr = 0;
        let mut count = nums.len();
        while count > 0 {
            let step = count / 2;
            let middle = curr + step;
            if nums[middle] < target {
                curr = middle + 1;
                count -= step + 1;
            } else {
                count = step;
            }
        }
        curr as i32
    }
}
fn main() {
    let ans = Solution::search_insert(vec![1, 3, 5, 6], 7);
    println!("{}", ans);
}
