use std::fmt::Debug;
fn main() {
    let v = vec![38, 27, 43, 10];

    println!("Final left side vector: {:?}", mergesort(&v));
}

fn mergesort<T: Ord + Clone + Debug>(arr: &[T]) -> Vec<T> {
    if arr.len() < 2 {
        return arr.to_vec();
    }
    let middle_index = arr.len() / 2;
    let left_arr = mergesort(&arr[..middle_index]);
    let right_arr = mergesort(&arr[middle_index..]);

    merge(&left_arr, &right_arr)
}
fn merge<T: Ord + Clone>(left_arr: &[T], right_arr: &[T]) -> Vec<T> {
    let mut merged_vec = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < left_arr.len() && j < right_arr.len() {
        if left_arr[i] <= right_arr[j] {
            merged_vec.push(left_arr[i].clone());
            i += 1;
        } else {
            merged_vec.push(right_arr[j].clone());
            j += 1;
        }
    }
    // Append any remaining elements from left_arr
    merged_vec.extend_from_slice(&left_arr[i..]);

    // Append any remaining elements from right_arr
    merged_vec.extend_from_slice(&right_arr[j..]);
    merged_vec
}
