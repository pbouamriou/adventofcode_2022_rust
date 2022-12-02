use std::io;

#[derive(Clone)]
pub struct Elf {
    pub number: u32,
    pub calories: u32,
}

impl Elf {
    pub fn new(number: u32) -> Self {
        Self {
            number,
            calories: 0,
        }
    }

    pub fn add_calories(&mut self, calories: u32) {
        self.calories += calories;
    }
}

pub struct Expedition {
    pub elves: Vec<Elf>,
}

impl Expedition {
    pub fn create_from_calorie_input(
        lines: &mut dyn Iterator<Item = Result<String, io::Error>>,
    ) -> Self {
        let mut elves = vec![];
        let mut elf = Elf::new(1);
        for line in lines {
            match line {
                Ok(str_line) => match str_line.parse::<u32>() {
                    Ok(calorie) => elf.add_calories(calorie),
                    Err(_) => {
                        let current_elf_number = elf.number;
                        elves.push(elf);
                        elf = Elf::new(current_elf_number + 1)
                    }
                },
                Err(_) => elves.push(elf.clone()),
            };
        }
        Self { elves }
    }
}
