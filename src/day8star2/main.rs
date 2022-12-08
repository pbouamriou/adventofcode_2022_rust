use std::io;

fn main() {
    use adventofcode::day8::*;
    let mut lines = io::stdin().lines();
    println!(
        "Best position score = {}",
        TreeMap::parse(&mut lines).best_position_score()
    );
}
