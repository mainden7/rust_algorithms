fn bubble_sort(mut array: Vec<i32>) -> Vec<i32> {
    for i in 0..array.len() {
        for j in i..array.len() {
            if array[j] < array[i] {
                array.swap(i, j);
            }
        }
    }
    array
}
fn main() {
    let ans = bubble_sort(vec![3, 3, 3, 56, 1, 2, 5, 6, 4, 3, 12, 90, 78]);
}
