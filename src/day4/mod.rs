use std::io;
use std::ops::RangeInclusive;

struct SectionAssignmentRange {
    range: RangeInclusive<u32>,
}

impl SectionAssignmentRange {
    fn is_in_range(&self, range: &SectionAssignmentRange) -> bool {
        let mut in_range = true;
        for value in self.range.clone() {
            in_range = in_range && range.range.contains(&value);
        }
        in_range
    }

    fn has_number_overlap(&self, range: &SectionAssignmentRange) -> bool {
        let mut overlap = false;
        for value in self.range.clone() {
            overlap = overlap || range.range.contains(&value);
        }
        overlap
    }

    fn parse(range_as_str: &str) -> SectionAssignmentRange {
        let mut parts = range_as_str.split("-");
        let range = parts.next().unwrap().parse::<u32>().unwrap()
            ..=parts.next().unwrap().parse::<u32>().unwrap();
        SectionAssignmentRange { range }
    }
}

struct AssignmentPair(SectionAssignmentRange, SectionAssignmentRange);

impl AssignmentPair {
    fn is_fully_contained(&self) -> bool {
        self.0.is_in_range(&self.1) || self.1.is_in_range(&self.0)
    }

    fn has_overlap(&self) -> bool {
        if self.0.range.clone().count() > self.1.range.clone().count() {
            self.0.has_number_overlap(&self.1)
        } else {
            self.1.has_number_overlap(&self.0)
        }
    }
}

pub struct SectionAssignments {
    assignments: Vec<AssignmentPair>,
}

impl SectionAssignments {
    pub fn parse(lines: &mut dyn Iterator<Item = Result<String, io::Error>>) -> Self {
        let mut assignments = vec![];
        for line in lines {
            if let Ok(line) = line {
                let mut parts = line.split(",");
                let part1 = SectionAssignmentRange::parse(parts.next().unwrap());
                let part2 = SectionAssignmentRange::parse(parts.next().unwrap());
                assignments.push(AssignmentPair(part1, part2));
            }
        }

        SectionAssignments { assignments }
    }

    pub fn number_of_assignment_pairs_fully_contained(&self) -> u32 {
        self.assignments.iter().fold(0, |accu, assignment_pair| {
            accu + {
                if assignment_pair.is_fully_contained() {
                    1
                } else {
                    0
                }
            }
        })
    }

    pub fn number_of_assignment_pairs_overlaps(&self) -> u32 {
        self.assignments.iter().fold(0, |accu, assignment_pair| {
            accu + {
                if assignment_pair.has_overlap() {
                    1
                } else {
                    0
                }
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testtools::*;

    #[test]
    fn test_fully_contained() {
        let data = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
            .to_string();
        let mut lines = read_from_string(&data);
        assert_eq!(
            SectionAssignments::parse(&mut lines).number_of_assignment_pairs_fully_contained(),
            2
        );
    }

    #[test]
    fn test_overlap() {
        let data = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
            .to_string();
        let mut lines = read_from_string(&data);
        assert_eq!(
            SectionAssignments::parse(&mut lines).number_of_assignment_pairs_overlaps(),
            4
        );
    }
}
