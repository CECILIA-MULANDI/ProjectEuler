fn main() {
    println!("{:?}", summation());
}
fn summation() -> u32 {
    let mut count = 0;
    let mut n = 10;
    let mut sum = 0;
    while count < 11 {
        if is_prime(n) {
            let (left_to_right, right_to_left) = truncate(n);

            // check if the truncated numbers are prime
            if left_to_right.iter().all(|&num| is_prime(num))
                && right_to_left.iter().all(|&num| is_prime(num))
            {
                sum += n;
                count += 1;
            }
        }
        n += 1;
    }
    sum
}
fn truncate(n: u32) -> (Vec<u32>, Vec<u32>) {
    let mut right_to_left = Vec::new();
    let mut left_to_right = Vec::new();
    let mut temp = n; // Preserve original value of n
    let mut divisor = 10;

    // Generate right-to-left truncations
    while temp != 0 {
        right_to_left.push(temp);
        temp /= 10;
    }

    // Generate left-to-right truncations
    temp = n; // Reset temp to original value of n
    while divisor <= temp * 10 {
        left_to_right.push(temp % divisor);
        divisor *= 10;
    }

    left_to_right.reverse();
    (right_to_left, left_to_right)
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false; // 1 and numbers less than 1 are not prime
    }
    if n == 2 {
        return true; // 2 is prime
    }
    if n % 2 == 0 {
        return false; // Even numbers greater than 2 are not prime
    }
    let limit = (n as f64).sqrt() as u32 + 1;
    for i in 3..limit {
        if n % i == 0 {
            return false; // Found a divisor, not prime
        }
    }
    true
}
