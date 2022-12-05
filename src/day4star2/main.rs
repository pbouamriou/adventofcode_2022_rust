use std::io;

fn main() {
    use adventofcode::day4::*;
    let mut lines = io::stdin().lines();

    let sum = SectionAssignments::parse(&mut lines).number_of_assignment_pairs_overlaps();
    println!("Nb of assignment pairs overlaps = {}", sum);
}
