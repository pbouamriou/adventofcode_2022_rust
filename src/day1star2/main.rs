use std::io;

fn main() {
    use adventofcode::day1::Elf;
    use adventofcode::day1::Expedition;
    let mut lines = io::stdin().lines();
    let expedition = Expedition::create_from_calorie_input(&mut lines);
    let total_calories_for_top_three_elf_carrying_most_calories = expedition
        .elves
        .into_iter()
        .fold(Vec::<Elf>::new(), |accumulator, elem| {
            let mut accumulator = accumulator;
            if accumulator.len() < 3 {
                accumulator.push(elem);
            } else {
                match accumulator
                    .iter()
                    .position(|accu_elem| elem.calories > accu_elem.calories)
                {
                    Some(position) => {
                        accumulator.remove(position);
                        accumulator.push(elem);
                    }
                    None => {}
                };
            }
            accumulator
        })
        .iter()
        .fold(0, |accu, elem| accu + elem.calories);
    println!(
        "Total calories for top three elf carrying most calories = {}",
        total_calories_for_top_three_elf_carrying_most_calories
    );
}
