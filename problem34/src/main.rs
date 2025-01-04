fn main() {
    println!(
        "The sum of all curious numbers is {:?}",
        find_curious_numbers()
    );
}

fn factorials() -> Vec<u32> {
    // 1. find factorials for digits between 1 and 9
    let mut factorial = vec![1; 10];
    // println!("{:?}", factorial);
    for i in 1..10 {
        factorial[i] = factorial[i - 1] * i as u32;
    }
    factorial
}
fn digit_factorial_sum(mut number: u32, factorials: &Vec<u32>) -> u32 {
    let mut sum = 0;
    while number > 0 {
        let digit = number % 10;
        sum += factorials[digit as usize];
        number /= 10;
    }
    sum
}

fn find_curious_numbers() -> u32 {
    let factorial = factorials();
    let upper_limit = 7 * factorial[9];
    let mut sum_of_curious_numbers = 0;

    for num in 10..=upper_limit {
        if num == digit_factorial_sum(num, &factorial) {
            sum_of_curious_numbers += num;
        }
    }
    sum_of_curious_numbers
}
