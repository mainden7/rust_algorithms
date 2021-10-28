fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() == 1 {
        return arr;
    }

    let mid = arr.len() / 2;

    let left = merge_sort(arr[..mid].to_vec());
    let right = merge_sort(arr[mid..].to_vec());

    return merge(left, right);
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);
    let mut out: Vec<i32> = vec![];
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            out.push(left[i]);
            i += 1;
        } else {
            out.push(right[j]);
            j += 1
        }
    }
    out.append(left[i..].to_vec().as_mut());
    out.append(right[j..].to_vec().as_mut());
    out
}

fn main() {
    let ans = merge_sort(vec![0, 5, 3, 6, 2, 1, 3, 4]);
    println!("{:?}", ans);
}
