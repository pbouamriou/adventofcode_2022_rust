use std::io;

#[derive(Clone)]
struct Elf {
    number: u32,
    calories: u32,
}

impl Elf {
    fn new(number: u32) -> Self {
        Self {
            number,
            calories: 0,
        }
    }

    fn add_calories(&mut self, calories: u32) {
        self.calories += calories;
    }
}

struct Expedition {
    elves: Vec<Elf>,
}

impl Expedition {
    fn create_from_calorie_input(
        lines: &mut dyn Iterator<Item = Result<String, io::Error>>,
    ) -> Self {
        let mut elves = vec![];
        let mut elf = Elf::new(1);
        for line in lines {
            match line {
                Ok(str_line) => match str_line.parse::<u32>() {
                    Ok(calorie) => elf.add_calories(calorie),
                    Err(_) => {
                        elves.push(elf.clone());
                        elf = Elf::new(elf.number + 1)
                    }
                },
                Err(_) => elves.push(elf.clone()),
            };
        }
        Self { elves }
    }
}

fn main() {
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
