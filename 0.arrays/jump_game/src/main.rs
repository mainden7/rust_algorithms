struct Solution {}

impl Solution {

    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut moves: i32 = 0;
        for (idx, step) in nums.iter().enumerate() {
            if idx as i32 > moves {
                return false;
            } else {
                let v = idx as i32 + step;
                moves = moves.max(v);
            }

        }
        true

    }
}

fn main() {
    let ans = Solution::can_jump(vec![2, 3, 1, 1, 4]);
    println!("{}", ans);
}
