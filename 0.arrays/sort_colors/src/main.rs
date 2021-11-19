pub fn sort_colors(nums: &mut Vec<i32>) {
    let (mut x, mut y) = (0, 0);
    for item in nums.iter() {
        if item == &0 {
            x += 1;
        } else if item == &1 {
            y += 1;
        }
    }

    for i in 0..nums.len() {
        if x > 0 {
            nums[i] = 0;
            x -= 1;
        } else if y > 0 {
            nums[i] = 1;
            y -= 1;
        } else {
            nums[i] = 2;
        }
    }
}

pub fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums);
    println!("{:?}", nums);
}
