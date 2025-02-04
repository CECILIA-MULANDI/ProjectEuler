fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let start = 0;
    let end = v.len() - 1;
    let target = 8;
    let res = binary_search(&v, start, end, target);
    println!("Target found at position:{:?}", res)
}
fn binary_search(arr: &[i32], start: usize, end: usize, target: i32) -> Option<usize> {
    if start > end {
        return None; // Base case: target not found
    }
    let mid = (start + end) / 2;
    if arr[mid] == target {
        Some(mid)
    } else if arr[mid] > target {
        return binary_search(arr, start, mid.saturating_sub(1), target);
    } else {
        return binary_search(arr, mid + 1, end, target);
    }
}
