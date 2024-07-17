// considering 1 and 2 to be the the first and second fibonacci numbers respectively
// find the sum of even fibonacci numbers that don't exceed 4million
// note: we are not generating 4Million fib numbers, we want that the last fib number does not exceed 4million

fn main() {
    let mut prev2 =1;
    let mut prev1 = 2;
   
    let mut sum = 2;
    
    while sum < 4_000_000 {     
        let res = prev2 + prev1;
       if res %2==0{
        sum+=res;

       }
        prev2 = prev1;
        prev1 = res;
    }
   
    
    println!("The sum:{}",sum)
}
// 4613732