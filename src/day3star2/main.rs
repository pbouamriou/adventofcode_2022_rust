use std::io;

fn main() {
    use adventofcode::day3::*;
    let mut lines = io::stdin().lines();

    let sum = SafetySupplies::parse(&mut lines).priorities_sum();
    println!("Sum of common badges priorities = {}", sum);
}
