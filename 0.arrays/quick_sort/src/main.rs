fn quick_sort(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() <= 1 {
        return nums;
    }
    let pivot = nums[0];
    let eq: Vec<i32> = nums.iter().filter(|&x| x == &pivot).cloned().collect();
    let lower: Vec<i32> = nums.iter().filter(|&x| x < &pivot).cloned().collect();
    let higher: Vec<i32> = nums.iter().filter(|&x| x > &pivot).cloned().collect();

    let mut res = quick_sort(lower)
        .into_iter()
        .chain(eq)
        .chain(quick_sort(higher))
        .collect();
    res
}

fn main() {
    let ans = quick_sort(vec![1]);
    println!("{:?}", ans);
}
