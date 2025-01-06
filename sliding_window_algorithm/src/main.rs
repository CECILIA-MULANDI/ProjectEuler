fn main() {
    let arr = [1, 2, 6, 2, 4, 1];
    println!("Largest sum is: {:?}", largest_sum(&arr));
    println!(
        "Largest sum is: {:?}",
        largest_sum_using_sliding_window(&arr, 3)
    );
}

// Array = [1, 2, 6, 2, 4, 1], k = 3
// naive
fn largest_sum(arr: &[u32]) -> u32 {
    let mut largest_sum = 0;
    for i in 0..arr.len() - 2 {
        for j in i + 1..arr.len() - 1 {
            for k in j + 1..arr.len() {
                let sum = arr[i] + arr[j] + arr[k];
                // println!("{sum}");
                if sum > largest_sum {
                    largest_sum = sum
                }
            }
        }
    }
    largest_sum
}
//  O(n)

// Array = [1, 2, 6, 2, 4, 1], k = 3
fn largest_sum_using_sliding_window(arr: &[u32], window_size: usize) -> u32 {
    if arr.len() < window_size {
        return 0;
    }
    let mut current_sum = 0;
    // Calculate the sum of the first 'window_size' elements
    for i in 0..window_size {
        current_sum += arr[i];
    }
    let mut largest_sum = current_sum;
    // Slide the window
    for i in window_size..arr.len() {
        // println!("{:?}", arr[i]);
        // Subtract the element going out of the window and add the element coming in
        current_sum = current_sum - arr[i - window_size] + arr[i];
        println!("{:?}", arr[i - window_size]);
        largest_sum = largest_sum.max(current_sum); // Update the largest sum if necessary
    }
    largest_sum
}
