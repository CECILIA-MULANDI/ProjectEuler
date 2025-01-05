use std::collections::HashSet;
fn main() {
    // let n = 220;
    // let (sum_of_divisors_n, sum_of_divisors_sum) = amicable_numbers(n);
    // println!(
    //     "The d({n}) is {sum_of_divisors_n} and d({sum_of_divisors_n}) is {sum_of_divisors_sum}"
    // );
    println!(
        "Sum of all amicable number below 10000 is {:?}",
        sum_of_all_amicable_number()
    )
}
fn amicable_numbers(n: u32) -> (u32, u32) {
    //220 ->
    let sum_of_divisors_of_n = sum_of_divisors(n);
    let sum_of_divisor_sum = sum_of_divisors(sum_of_divisors_of_n);
    (sum_of_divisors_of_n, sum_of_divisor_sum)
}

fn sum_of_divisors(m: u32) -> u32 {
    (1..m).filter(|x| m % x == 0).sum()
}
fn is_amicable(n: u32) -> bool {
    let (sum_of_divisors_n, sum_of_divisors_sum) = amicable_numbers(n);
    sum_of_divisors_sum == n && sum_of_divisors_n != n
}
fn sum_of_all_amicable_number() -> u32 {
    let mut count = 0;
    let mut sum = 0;
    let mut seen = HashSet::new();
    while count < 10000 {
        if seen.contains(&count) {
            count += 1;
            continue;
        }
        if is_amicable(count) {
            let (sum_of_divisors_n, sum_of_divisors_sum) = amicable_numbers(count);
            if !seen.contains(&sum_of_divisors_n) {
                sum += count;
                sum += sum_of_divisors_n;
                seen.insert(count);
                seen.insert(sum_of_divisors_n);
            }
        }
        count += 1
    }
    sum
}
