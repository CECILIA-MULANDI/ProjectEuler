fn main() {
    println!(
        "The number with the longest sequence is: {:?}",
        longest_collatz()
    );
}

fn longest_collatz() -> u32 {
    let mut current_longest = 0;
    let limit = 1_000_000;

    let mut number_with_longest = 0;
    for start in 1..limit {
        let mut i = start as u64;
        let mut length = 0;
        while i != 1 {
            if i % 2 == 0 {
                i = i / 2;
            } else {
                i = 3 * i + 1;
            }
            length += 1;
        }
        length += 1;
        if length > current_longest {
            current_longest = length;
            number_with_longest = start;
        }
    }
    println!("The longest sequence is {current_longest}");
    number_with_longest
}
