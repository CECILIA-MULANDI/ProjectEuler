fn main() {
    println!("{:?}", prime_pair_connection());
}

fn prime_pair_connection() -> u64 {
    let lower_bound: u64 = 5;
    let upper_bound: u64 = 1_000_000;
    let mut sum = 0;
    let mut consecutive_primes = Vec::new();

    for i in lower_bound..=upper_bound {
        if is_prime(i) {
            consecutive_primes.push(i);
            if consecutive_primes.len() == 2 {
                let s = find_s(&consecutive_primes);
                sum += s;
                // Keep only the last prime in consecutive_primes
                consecutive_primes.remove(0);
            }
        }
    }

    sum
}

fn find_s(primes: &Vec<u64>) -> u64 {
    let last_digits = primes[0]; // p1 is the number whose last digits we need to match
    let mod_value = 10_u64.pow((get_digit_count(last_digits) as u64).try_into().unwrap()); // 10^k where k is the number of digits in p1
    let mut s: u64 = last_digits;

    // Loop to find the smallest number that ends with last_digits and is divisible by p2
    while s % primes[1] != 0 {
        s += mod_value;
    }

    s
}

// Helper function to find the number of digits in a number
fn get_digit_count(n: u64) -> usize {
    n.to_string().len()
}

// Helper function to check if a number is prime
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt();
    for i in (3..=limit as u64).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
