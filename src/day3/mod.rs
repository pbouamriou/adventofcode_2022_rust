use std::io;

#[derive(Debug, PartialEq)]
struct Item(char);

impl Item {
    fn from_char(character: char) -> Option<Item> {
        match character {
            'a'..='z' | 'A'..='Z' => Some(Item(character)),
            _ => None,
        }
    }

    fn priority(&self) -> u32 {
        let code: u32 = self.0.into();
        match self.0 {
            'a'..='z' => code - u32::from('a') + 1,
            'A'..='Z' => code - u32::from('A') + 27,
            _ => 0,
        }
    }
}

#[derive(Debug)]
struct Compartment {
    items: Vec<Item>,
}

struct Rucksack {
    first_compartment: Compartment,
    second_compartment: Compartment,
}

impl Rucksack {
    fn from_line(line: &String) -> Self {
        let each_len = line.len() / 2;
        let (left, right) = line.split_at(each_len);
        let first_compartiment = Compartment {
            items: left
                .to_string()
                .chars()
                .fold(Vec::<Item>::new(), |accumulator, item| {
                    let mut accumulator = accumulator;
                    accumulator.push(Item::from_char(item).unwrap());
                    accumulator
                }),
        };
        let second_compartiment = Compartment {
            items: right
                .to_string()
                .chars()
                .fold(Vec::<Item>::new(), |accumulator, item| {
                    let mut accumulator = accumulator;
                    accumulator.push(Item::from_char(item).unwrap());
                    accumulator
                }),
        };
        Rucksack {
            first_compartment: first_compartiment,
            second_compartment: second_compartiment,
        }
    }

    fn find_common_item<'a>(&'a self) -> Option<&'a Item> {
        let mut found_item = None;
        for item_first_compartiment in &self.first_compartment.items {
            match self
                .second_compartment
                .items
                .iter()
                .find(|item| item.0 == item_first_compartiment.0)
            {
                Some(item) => {
                    found_item = Some(item);
                    break;
                }
                None => {}
            }
        }
        found_item
    }

    fn iter(&self) -> RucksackIterator {
        RucksackIterator {
            rucksack: &self,
            position: 0,
        }
    }
}

struct RucksackIterator<'a> {
    rucksack: &'a Rucksack,
    position: usize,
}

impl<'a> Iterator for RucksackIterator<'a> {
    type Item = &'a Item;
    fn next(&mut self) -> Option<Self::Item> {
        let len_first_compartment = self.rucksack.first_compartment.items.len();
        if self.position < len_first_compartment {
            let item = self
                .rucksack
                .first_compartment
                .items
                .iter()
                .nth(self.position);
            self.position += 1;
            item
        } else {
            let item = self
                .rucksack
                .second_compartment
                .items
                .iter()
                .nth(self.position - len_first_compartment);
            self.position += 1;
            item
        }
    }
}

pub struct ElvesGroup {
    rucksacks: Vec<Rucksack>,
}

impl ElvesGroup {
    fn find_badge<'a>(&'a self) -> Option<&'a Item> {
        let mut iter = self.rucksacks.iter();
        if let Some(firt_rucksacks) = iter.next() {
            for item_to_found in firt_rucksacks.iter() {
                let other_rucksacks = self
                    .rucksacks
                    .iter()
                    .enumerate()
                    .filter(|&(index, _)| index != 0);
                let mut found = true;
                for (_, rucksack) in other_rucksacks {
                    let item_found = rucksack.iter().find(|item| item.0 == item_to_found.0);
                    match item_found {
                        Some(_) => {}
                        None => {
                            found = false;
                            break;
                        }
                    };
                }
                if found {
                    return Some(item_to_found);
                }
            }
        }
        None
    }
}

pub struct SafetySupplies {
    groups: Vec<ElvesGroup>,
}

impl SafetySupplies {
    pub fn parse(lines: &mut dyn Iterator<Item = Result<String, io::Error>>) -> SafetySupplies {
        let mut groups = vec![];
        let mut rucksacks = vec![];
        let mut group_number = 0;
        for line in lines {
            let rucksack = Rucksack::from_line(&line.unwrap());
            rucksacks.push(rucksack);
            if group_number == 2 {
                groups.push(ElvesGroup { rucksacks });
                rucksacks = vec![];
                group_number = 0;
            } else {
                group_number += 1;
            }
        }
        if rucksacks.len() > 0 {
            groups.push(ElvesGroup { rucksacks });
        }

        SafetySupplies { groups }
    }

    pub fn priorities_sum(&self) -> u32 {
        self.groups.iter().fold(0, |accumulator, group| {
            accumulator + group.find_badge().unwrap().priority()
        })
    }
}

pub struct Supplies {
    rucksacks: Vec<Rucksack>,
}

impl Supplies {
    pub fn parse(lines: &mut dyn Iterator<Item = Result<String, io::Error>>) -> Supplies {
        let mut rucksacks = vec![];
        for line in lines {
            let rucksack = Rucksack::from_line(&line.unwrap());
            rucksacks.push(rucksack);
        }
        Supplies { rucksacks }
    }

    pub fn priorities_sum(&self) -> u32 {
        self.rucksacks.iter().fold(0, |accumulator, rucksack| {
            accumulator + rucksack.find_common_item().unwrap().priority()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testtools::*;

    #[test]
    fn test_priority() {
        assert_eq!(Item::from_char('a').unwrap().priority(), 1);
        assert_eq!(Item::from_char('z').unwrap().priority(), 26);
        assert_eq!(Item::from_char('A').unwrap().priority(), 27);
        assert_eq!(Item::from_char('Z').unwrap().priority(), 52);
    }

    #[test]
    fn test_priority_sum() {
        let data = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
            .to_string();
        let mut lines = read_from_string(&data);
        assert_eq!(Supplies::parse(&mut lines).priorities_sum(), 157);
    }

    #[test]
    fn test_rucksack_iterator() {
        let rucksack = Rucksack::from_line(&"vJrwpWtwJgWrhcsFMMfFFhFp".to_string());
        assert_eq!(
            rucksack
                .iter()
                .collect::<Vec<&Item>>()
                .iter()
                .fold("".to_string(), |accu, item| {
                    let mut accu = accu;
                    accu.push(item.0);
                    accu
                }),
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string()
        );
    }

    #[test]
    fn test_badge_priority_sum() {
        let data = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
            .to_string();
        let mut lines = read_from_string(&data);
        assert_eq!(SafetySupplies::parse(&mut lines).priorities_sum(), 70);
    }
}
