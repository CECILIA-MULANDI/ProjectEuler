


fn main(){
    // println!(" {}",is_prime(600851475143));
    let res=find_factors(600851475143);
    // let res=find_factors(10);
    println!("{:?}",res);

    println!("{}",find_largest(&res))
    
}

fn is_prime(n:i64)->bool{
    let mut flag=true;
    if n <= 1 {
        flag=false;
    }
    for i in 2..=(n as f64).sqrt() as i64 {
        if n%i==0{
            flag=false
        }
    }
    flag

}
// how to find factors of a number
// 10 -2*5

fn find_factors(n:u128)->Vec<u128>{
    let mut res =Vec::new();
    // Calculate the integer square root of n
    let limit = (n as f64).sqrt() as u128;
    for i in 2..=limit{
        if n%i==0{
            if  is_prime(i as i64){
                res.push(i)
            }
            let pair_factor = n / i;
            
            // Check if the pair factor is different from i and is prime
            if pair_factor != i && is_prime(pair_factor as i64) {
                // If the pair factor is prime, add it to the result vector
                res.push(pair_factor);
            }
            
            

        }
        
    }
    res

}
fn find_largest(res: &[u128]) -> u128 {
    *res.iter().max().unwrap()
}