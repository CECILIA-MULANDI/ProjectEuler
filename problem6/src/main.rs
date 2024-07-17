fn main() {
    println!("{:?}",find_squares());
    println!("{:?}",square_of_sum());
    let difference = square_of_sum()-find_squares();
    println!("The difference is: {}",difference)
}
fn find_squares()->u64{
    (1..=100).map(|i| i*i).sum()
    
}
fn square_of_sum()->u64{
    let sum:u64=(1..=100).sum();
    
    sum*sum
}
// 25164150