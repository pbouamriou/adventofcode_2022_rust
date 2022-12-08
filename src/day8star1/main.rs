use std::io;

fn main() {
    use adventofcode::day8::*;
    let mut lines = io::stdin().lines();
    println!(
        "Number of trees visible {}",
        TreeMap::parse(&mut lines).how_many_trees_visible()
    );
}
