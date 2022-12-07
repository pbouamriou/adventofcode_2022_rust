use std::io;

fn main() {
    use adventofcode::day6::*;
    let mut lines = io::stdin().lines();

    println!(
        "Start of packet : {}",
        find_start_of_message(&mut lines).unwrap()
    );
}
