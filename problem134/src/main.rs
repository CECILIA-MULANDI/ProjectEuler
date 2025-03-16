fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut sieve = vec![true; limit + 1];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..=(limit as f64).sqrt() as usize {
        if sieve[i] {
            for j in (i * i..=limit).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(i, is_prime)| if is_prime { Some(i) } else { None })
        .collect()
}

fn find_smallest_s(p1: usize, p2: usize) -> usize {
    let p1_len = p1.to_string().len(); // Get the length of p1
    let mut n = p1; // Start with p1

    loop {
        // Check if the current number n is divisible by p2
        if n % p2 == 0 {
            return n;
        }
        // Increase n by 10^len(p1) to ensure it ends in p1
        n += 10_usize.pow(p1_len as u32);
    }
}

fn main() {
    let limit = 1_000_000; // Limit for primes generation

    // Generate all primes up to the limit
    let primes = sieve_of_eratosthenes(limit);

    let mut total_sum = 0;

    // Iterate over consecutive pairs of primes
    for i in 1..primes.len() {
        let p1 = primes[i]; // Current prime
        let p2 = primes[i - 1]; // Previous prime

        total_sum += find_smallest_s(p1, p2); // Find the smallest S for this pair
    }

    // Print the total sum of all smallest S values
    println!("The sum of S is: {}", total_sum);
}
