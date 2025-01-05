fn main() {
    let n = 220;
    println!(
        "The sum of the divisors of {n} is {:?}",
        amicable_numbers(n)
    );
}
fn amicable_numbers(n: u32) -> (u32, u32) {
    //220 ->
    let mut sum = 0;

    for i in 1..n {
        if n % i == 0 {
            sum += i;
        }
    }
    // sum
    let mut new_sum = sum;
    let mut sum_of_sum = 0;
    for i in 1..new_sum {
        if new_sum % i == 0 {
            sum_of_sum += i;
        }
    }
    (sum, sum_of_sum)
}
