use std::io;

fn main() {
    use adventofcode::day7::*;
    let mut lines = io::stdin().lines();
    let fs = FileSystem::make_from_listing(&mut lines);
    println!("Total size of directorie = {}", fs.total_size_directories());
}
