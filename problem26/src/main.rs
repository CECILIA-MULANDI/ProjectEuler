use std::collections::HashMap;
fn main() {
    let mut max_len = 0;
    let mut res_d = 0;
    for d in 2..1000 {
        if d % 2 == 0 || d % 5 == 0 {
            continue;
        }
        let cycle_len = reciprocal_cycle(d);
        if cycle_len > max_len {
            max_len = cycle_len;
            res_d = d;
        }
    }
    println!(
        "The value of d < 1000 with the longest recurring cycle is: {}",
        res_d
    );
}

fn reciprocal_cycle(d: usize) -> usize {
    let mut remainder = HashMap::new();
    let mut value = 1;
    let mut position: usize = 0;

    while value != 0 {
        if let Some(&startposition) = remainder.get(&value) {
            return position - startposition;
        }
        remainder.insert(value, position);
        value = (value * 10) % d;
        position += 1;
    }
    0
}
