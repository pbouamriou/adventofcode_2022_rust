use std::io;
fn main() {
    use adventofcode::day5::*;
    let mut lines = io::stdin().lines();

    let mut supply_stacks = SupplyStacks::parse(&mut lines);
    supply_stacks.parse_move(&mut lines, &MoveMethod::Reorder);
    println!("Top of each stack = {}", supply_stacks.top_of_each_stack());
}
