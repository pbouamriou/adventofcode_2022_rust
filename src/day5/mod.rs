use std::collections::VecDeque;
use std::io;
use std::str::Chars;

#[derive(Debug, PartialEq)]
struct Crate {
    id: char,
}

#[derive(Debug, PartialEq)]
struct Stack {
    crate_stack: Vec<Crate>,
}

impl Stack {
    fn pop(&mut self) -> Option<Crate> {
        self.crate_stack.pop()
    }

    fn push(&mut self, crate_element: &Crate) {
        self.crate_stack.push(Crate {
            id: crate_element.id,
        });
    }

    fn top(&self) -> Option<&Crate> {
        self.crate_stack.last()
    }
}

pub struct SupplyStacks {
    stacks: Vec<Stack>,
}

#[derive(Debug)]
enum HeaderToken {
    Crate(char),
    Empty,
    Index(usize),
    EndOfLine,
    Error,
}

#[derive(Debug)]
enum ParseTokenState {
    Start,
    BeginCrate,
    InCrate(char),
    EndCrate(char),
    BeginIndex(usize),
    EndIndex(usize),
    EndLine,
}

#[derive(Debug, PartialEq)]
enum LineType {
    Crate(Vec<Option<char>>),
    Index(usize),
    Empty,
    Other,
}

pub enum MoveMethod {
    Reorder,
    KeepOrder,
}

impl SupplyStacks {
    pub fn parse(lines: &mut dyn Iterator<Item = Result<String, io::Error>>) -> Self {
        let stacks = Self::parse_header_lines(lines);

        SupplyStacks { stacks }
    }

    pub fn parse_move(
        &mut self,
        lines: &mut dyn Iterator<Item = Result<String, io::Error>>,
        move_method: &MoveMethod,
    ) {
        let regex_move =
            regex::Regex::new(r"move\s*(?P<move>\d+)\s*from\s*(?P<from>\d+)\s*to\s*(?P<to>\d+)")
                .unwrap();
        for line in lines {
            if let Ok(line) = line {
                let names = regex_move.captures(&line);
                if let Some(names) = names {
                    let number_of_moves =
                        names.name("move").unwrap().as_str().parse::<u32>().unwrap();
                    let from = names
                        .name("from")
                        .unwrap()
                        .as_str()
                        .parse::<usize>()
                        .unwrap();
                    let to = names.name("to").unwrap().as_str().parse::<usize>().unwrap();
                    self.apply_move(number_of_moves, from - 1, to - 1, &move_method);
                }
            }
        }
    }

    pub fn apply_move(
        &mut self,
        number_of_moves: u32,
        from: usize,
        to: usize,
        move_method: &MoveMethod,
    ) {
        match move_method {
            MoveMethod::Reorder => {
                for _ in 1..=number_of_moves {
                    let from_element = {
                        let from = self.stacks.get_mut(from).unwrap();
                        Some(from.pop().unwrap())
                    };

                    let to = self.stacks.get_mut(to).unwrap();
                    to.push(&from_element.unwrap());
                }
            }
            MoveMethod::KeepOrder => {
                let mut tmp_vec = VecDeque::<Crate>::new();
                for _ in 1..=number_of_moves {
                    let from_element = {
                        let from = self.stacks.get_mut(from).unwrap();
                        Some(from.pop().unwrap())
                    };
                    tmp_vec.push_back(from_element.unwrap());
                }

                for _ in 1..=number_of_moves {
                    let element = tmp_vec.pop_back().unwrap();
                    let to = self.stacks.get_mut(to).unwrap();
                    to.push(&element);
                }
            }
        }
    }

    pub fn top_of_each_stack(&self) -> String {
        let mut result = String::new();
        for stack in self.stacks.iter() {
            let top_element = stack.top().unwrap().id;
            result.push(top_element);
        }

        result
    }

    fn parse_header_lines(
        lines: &mut dyn Iterator<Item = Result<String, io::Error>>,
    ) -> Vec<Stack> {
        let mut dimension: usize = 0;
        let mut temp_vec = vec![];
        for line in lines {
            if let Ok(mut line) = line {
                let token = Self::parse_header_line(&mut line);
                match token {
                    LineType::Crate(crate_vec) => {
                        temp_vec.push(crate_vec);
                    }
                    LineType::Index(index) => dimension = index,
                    LineType::Empty => break,
                    _ => {}
                }
            }
        }

        let mut stacks = Vec::<Stack>::new();
        let max_height = temp_vec.len();
        for stack_id in 0..dimension {
            let mut stack = Stack {
                crate_stack: vec![],
            };
            for height in 0..max_height {
                if let Some(element) = temp_vec
                    .get(max_height - height - 1)
                    .unwrap()
                    .get(stack_id)
                    .unwrap()
                {
                    stack.crate_stack.push(Crate { id: *element });
                }
            }
            stacks.push(stack);
        }

        stacks
    }

    fn parse_header_token(chars: &mut Chars) -> HeaderToken {
        let mut token_state = ParseTokenState::Start;
        for _ in 0..=2 {
            if let Some(char) = chars.next() {
                match token_state {
                    ParseTokenState::Start => match char {
                        '[' => token_state = ParseTokenState::BeginCrate,
                        '0'..='9' => {
                            token_state = ParseTokenState::BeginIndex(
                                usize::try_from(u32::from(char) - u32::from('0')).unwrap(),
                            )
                        }
                        _ => {}
                    },
                    ParseTokenState::BeginCrate => match char {
                        'A'..='Z' => token_state = ParseTokenState::InCrate(char),
                        _ => {}
                    },
                    ParseTokenState::InCrate(crate_id) => match char {
                        ']' => token_state = ParseTokenState::EndCrate(crate_id),
                        _ => {}
                    },
                    ParseTokenState::BeginIndex(number) => match char {
                        ' ' => token_state = ParseTokenState::EndIndex(number),
                        _ => {}
                    },
                    ParseTokenState::EndLine => {}
                    ParseTokenState::EndIndex(_) => {}
                    ParseTokenState::EndCrate(_) => {}
                }
            } else {
                token_state = ParseTokenState::EndLine;
            }
        }
        match token_state {
            ParseTokenState::Start => HeaderToken::Empty,
            ParseTokenState::EndCrate(crate_id) => HeaderToken::Crate(crate_id),
            ParseTokenState::EndIndex(number) => HeaderToken::Index(number),
            ParseTokenState::EndLine => HeaderToken::EndOfLine,
            _ => HeaderToken::Error,
        }
    }

    fn parse_header_line(line: &str) -> LineType {
        let mut result = LineType::Other;
        let mut chars = line.chars();
        while {
            let token = Self::parse_header_token(&mut chars);
            match token {
                HeaderToken::EndOfLine => result = LineType::Empty,
                HeaderToken::Crate(crate_id) => {
                    if result == LineType::Other {
                        result = LineType::Crate(vec![Some(crate_id)]);
                    } else if let LineType::Crate(ref mut crates) = result {
                        crates.push(Some(crate_id));
                    }
                }
                HeaderToken::Empty => {
                    if result == LineType::Other {
                        result = LineType::Crate(vec![None]);
                    } else if let LineType::Crate(ref mut crates) = result {
                        crates.push(None);
                    }
                }
                HeaderToken::Index(index) => {
                    if result == LineType::Other {
                        result = LineType::Index(index);
                    } else if let LineType::Index(_) = result {
                        result = LineType::Index(index);
                    }
                }
                _ => {}
            }
            if let Some(_) = chars.next() {
                true
            } else {
                false
            }
        } {}
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testtools::*;

    #[test]
    fn test_parse_header_line() {
        assert_eq!(
            SupplyStacks::parse_header_line("[Z] [F]     [G] [Q] [F]            "),
            LineType::Crate(vec![
                Some('Z'),
                Some('F'),
                None,
                Some('G'),
                Some('Q'),
                Some('F'),
                None,
                None,
                None
            ])
        );
        assert_eq!(
            SupplyStacks::parse_header_line("[J]             [F] [M]            "),
            LineType::Crate(vec![
                Some('J'),
                None,
                None,
                None,
                Some('F'),
                Some('M'),
                None,
                None,
                None
            ])
        );
        assert_eq!(
            SupplyStacks::parse_header_line(" 1   2   3   4   5   6   7   8   9 "),
            LineType::Index(9)
        );
        assert_eq!(SupplyStacks::parse_header_line(""), LineType::Empty);
    }

    #[test]
    fn test_parse_header_lines() {
        let header = r#"[J]             [F] [M]            
[Z] [F]     [G] [Q] [F]            
[G] [P]     [H] [Z] [S] [Q]        
[V] [W] [Z] [P] [D] [G] [P]        
[T] [D] [S] [Z] [N] [W] [B] [N]    
[D] [M] [R] [J] [J] [P] [V] [P] [J]
[B] [R] [C] [T] [C] [V] [C] [B] [P]
[N] [S] [V] [R] [T] [N] [G] [Z] [W]
 1   2   3   4   5   6   7   8   9 
"#
        .to_string();
        let mut lines = read_from_string(&header);
        let supply_stacks = SupplyStacks::parse(&mut lines);
        let res = supply_stacks.stacks.get(0).unwrap();
        assert_eq!(
            *res,
            Stack {
                crate_stack: vec![
                    Crate { id: 'N' },
                    Crate { id: 'B' },
                    Crate { id: 'D' },
                    Crate { id: 'T' },
                    Crate { id: 'V' },
                    Crate { id: 'G' },
                    Crate { id: 'Z' },
                    Crate { id: 'J' }
                ]
            }
        );
        let res = supply_stacks.stacks.get(1).unwrap();
        assert_eq!(
            *res,
            Stack {
                crate_stack: vec![
                    Crate { id: 'S' },
                    Crate { id: 'R' },
                    Crate { id: 'M' },
                    Crate { id: 'D' },
                    Crate { id: 'W' },
                    Crate { id: 'P' },
                    Crate { id: 'F' }
                ]
            }
        );
        let res = supply_stacks.stacks.get(2).unwrap();
        assert_eq!(
            *res,
            Stack {
                crate_stack: vec![
                    Crate { id: 'V' },
                    Crate { id: 'C' },
                    Crate { id: 'R' },
                    Crate { id: 'S' },
                    Crate { id: 'Z' }
                ]
            }
        );
        let res = supply_stacks.stacks.get(7).unwrap();
        assert_eq!(
            *res,
            Stack {
                crate_stack: vec![
                    Crate { id: 'Z' },
                    Crate { id: 'B' },
                    Crate { id: 'P' },
                    Crate { id: 'N' }
                ]
            }
        );
        let res = supply_stacks.stacks.get(8).unwrap();
        assert_eq!(
            *res,
            Stack {
                crate_stack: vec![Crate { id: 'W' }, Crate { id: 'P' }, Crate { id: 'J' }]
            }
        );
    }

    #[test]
    fn test_parse_move_reorder() {
        let complete = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
            .to_string();
        let mut lines = read_from_string(&complete);
        let mut supply_stacks = SupplyStacks::parse(&mut lines);
        supply_stacks.parse_move(&mut lines, &MoveMethod::Reorder);
        let res = supply_stacks.stacks.get(0).unwrap();
        assert_eq!(
            *res,
            Stack {
                crate_stack: vec![Crate { id: 'C' }]
            }
        );
        let res = supply_stacks.stacks.get(1).unwrap();
        assert_eq!(
            *res,
            Stack {
                crate_stack: vec![Crate { id: 'M' },]
            }
        );
        let res = supply_stacks.stacks.get(2).unwrap();
        assert_eq!(
            *res,
            Stack {
                crate_stack: vec![
                    Crate { id: 'P' },
                    Crate { id: 'D' },
                    Crate { id: 'N' },
                    Crate { id: 'Z' }
                ]
            }
        );

        assert_eq!("CMZ".to_string(), supply_stacks.top_of_each_stack());
    }

    #[test]
    fn test_parse_move_keep_order() {
        let complete = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
            .to_string();
        let mut lines = read_from_string(&complete);
        let mut supply_stacks = SupplyStacks::parse(&mut lines);
        supply_stacks.parse_move(&mut lines, &MoveMethod::KeepOrder);
        let res = supply_stacks.stacks.get(0).unwrap();
        assert_eq!(
            *res,
            Stack {
                crate_stack: vec![Crate { id: 'M' }]
            }
        );
        let res = supply_stacks.stacks.get(1).unwrap();
        assert_eq!(
            *res,
            Stack {
                crate_stack: vec![Crate { id: 'C' },]
            }
        );
        let res = supply_stacks.stacks.get(2).unwrap();
        assert_eq!(
            *res,
            Stack {
                crate_stack: vec![
                    Crate { id: 'P' },
                    Crate { id: 'Z' },
                    Crate { id: 'N' },
                    Crate { id: 'D' }
                ]
            }
        );

        assert_eq!("MCD".to_string(), supply_stacks.top_of_each_stack());
    }
}
