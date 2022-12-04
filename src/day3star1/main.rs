use std::io;

fn main() {
    use adventofcode::day3::*;
    let mut lines = io::stdin().lines();

    let sum = Supplies::parse(&mut lines).priorities_sum();
    println!("Sum of common items priorities = {}", sum);
}
