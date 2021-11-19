struct Solution {}

impl Solution {
    pub fn search_sorted(matrix: &Vec<Vec<i32>>, target: i32) -> (i32, i32) {
        fn binary_search(nums: &Vec<i32>, target: &i32) -> i32 {

            let mut left = 0;
            let mut right = nums.len() - 1;

            while left <= right {
                let mid = left + (right - left) / 2;
                if &nums[mid] == target {
                    return mid as i32;
                } else if target < &nums[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            -1
        }
        for (idx, row) in matrix.iter().enumerate() {
            if target == row[0] {
                return (idx as i32, 0);
            } else if target == row[row.len() - 1] {
                return (idx as i32, (row.len() - 1) as i32);
            } else if target < row[0] || target > row[row.len() - 1] {
                continue;
            } else {
                let col_idx = binary_search(&row, &target);
                if col_idx != -1 {
                    return (idx as i32, col_idx);
                }
            }
        }
        (-1, -1)
    }
}

fn main() {
    let mat = vec![vec![1, 2, 3], vec![4, 5, 8], vec![8, 9, 10]];
    let ans = Solution::search_sorted(&mat, 7);
    println!("{:?}", ans);
}
