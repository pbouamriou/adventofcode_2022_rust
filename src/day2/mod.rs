use std::char;
use std::io;

pub struct Game {
    pub number_of_matches: u32,
    pub points: u32,
}

impl Game {
    pub fn parse_strategy_guide(
        lines: &mut dyn Iterator<Item = Result<String, io::Error>>,
    ) -> Self {
        let mut game = Self {
            number_of_matches: 0,
            points: 0,
        };
        for line in lines {
            match line {
                Ok(line) => {
                    let mut split = line.split(' ');
                    let first_letter = split.next().unwrap();
                    let second_letter = split.next().unwrap();
                    let opponent_choice =
                        IndividualChoice::parse(first_letter.chars().next().unwrap()).unwrap();
                    let my_choice =
                        IndividualChoice::parse(second_letter.chars().next().unwrap()).unwrap();
                    let points = RoundInfo::calculate_result(my_choice, opponent_choice).points();
                    game.points += points;
                    game.number_of_matches += 1;
                }
                Err(_) => {}
            }
        }
        game
    }

    pub fn parse_ultra_top_secret_strategy_guide(
        lines: &mut dyn Iterator<Item = Result<String, io::Error>>,
    ) -> Self {
        let mut game = Self {
            number_of_matches: 0,
            points: 0,
        };
        for line in lines {
            match line {
                Ok(line) => {
                    let mut split = line.split(' ');
                    let first_letter = split.next().unwrap();
                    let second_letter = split.next().unwrap();
                    let opponent_choice =
                        IndividualChoice::parse(first_letter.chars().next().unwrap()).unwrap();
                    let round_result =
                        RoundResult::parse(second_letter.chars().next().unwrap()).unwrap();
                    let my_choice =
                        IndividualChoice::make_my_choice(&opponent_choice, &round_result);
                    let points = RoundInfo::calculate_result(my_choice, opponent_choice).points();
                    game.points += points;
                    game.number_of_matches += 1;
                }
                Err(_) => {}
            }
        }
        game
    }
}

enum IndividualChoice {
    Paper,
    Scissors,
    Rock,
}

trait Evaluation {
    fn points(&self) -> u32;
}

impl Evaluation for IndividualChoice {
    fn points(&self) -> u32 {
        match self {
            Self::Paper => 2,
            Self::Scissors => 3,
            Self::Rock => 1,
        }
    }
}

impl IndividualChoice {
    fn parse(character: char) -> Option<Self> {
        match character {
            'A' => Some(Self::Rock),
            'B' => Some(Self::Paper),
            'C' => Some(Self::Scissors),
            'Y' => Some(Self::Paper),
            'X' => Some(Self::Rock),
            'Z' => Some(Self::Scissors),
            _ => None,
        }
    }

    fn make_my_choice(opponent_choice: &IndividualChoice, result: &RoundResult) -> Self {
        match opponent_choice {
            Self::Paper => match result {
                RoundResult::Win => Self::Scissors,
                RoundResult::Draw => Self::Paper,
                RoundResult::Loose => Self::Rock,
            },
            Self::Rock => match result {
                RoundResult::Win => Self::Paper,
                RoundResult::Draw => Self::Rock,
                RoundResult::Loose => Self::Scissors,
            },
            Self::Scissors => match result {
                RoundResult::Win => Self::Rock,
                RoundResult::Draw => Self::Scissors,
                RoundResult::Loose => Self::Paper,
            },
        }
    }
}

enum RoundResult {
    Win,
    Draw,
    Loose,
}

impl RoundResult {
    fn points(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loose => 0,
        }
    }

    fn parse(character: char) -> Option<Self> {
        match character {
            'X' => Some(Self::Loose),
            'Y' => Some(Self::Draw),
            'Z' => Some(Self::Win),
            _ => None,
        }
    }
}

struct RoundInfo {
    result: RoundResult,
    my_choice: Option<IndividualChoice>,
}

impl RoundInfo {
    fn calculate_result(my_choice: IndividualChoice, opponent_choice: IndividualChoice) -> Self {
        let result = match opponent_choice {
            IndividualChoice::Paper => match my_choice {
                IndividualChoice::Paper => RoundResult::Draw,
                IndividualChoice::Rock => RoundResult::Loose,
                IndividualChoice::Scissors => RoundResult::Win,
            },
            IndividualChoice::Rock => match my_choice {
                IndividualChoice::Paper => RoundResult::Win,
                IndividualChoice::Rock => RoundResult::Draw,
                IndividualChoice::Scissors => RoundResult::Loose,
            },
            IndividualChoice::Scissors => match my_choice {
                IndividualChoice::Paper => RoundResult::Loose,
                IndividualChoice::Rock => RoundResult::Win,
                IndividualChoice::Scissors => RoundResult::Draw,
            },
        };
        Self {
            result,
            my_choice: Some(my_choice),
        }
    }

    fn points(&self) -> u32 {
        let choice_points = match &self.my_choice {
            Some(choice) => choice.points(),
            None => 0,
        };
        self.result.points() + choice_points
    }
}
