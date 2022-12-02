use std::io;

fn main() {
    use adventofcode::day1::Expedition;
    let mut lines = io::stdin().lines();
    let expedition = Expedition::create_from_calorie_input(&mut lines);
    let elf_carrying_most_calories = expedition
        .elves
        .into_iter()
        .reduce(|accumulator, elem| {
            if accumulator.calories < elem.calories {
                elem
            } else {
                accumulator
            }
        })
        .unwrap();
    println!(
        "Elf carrying most calories is {}, total calories = {}",
        elf_carrying_most_calories.number, elf_carrying_most_calories.calories
    );
}
