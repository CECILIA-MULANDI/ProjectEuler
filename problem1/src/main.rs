fn main() {
    // 3 or 5=3,5,6,9 for numbers below 10
    // sum is 23
    let mut sum =0;
    for i in 1..1000{
       if i%3==0 || i% 5==0{
            sum+=i;
       }
    }
    println!("the sum of multiples of 3 and 5 below 10 is {}",sum);
}
