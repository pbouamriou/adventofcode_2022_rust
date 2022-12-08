use std::io;

pub struct TreeMap {
    map: Vec<Vec<u32>>,
}

impl TreeMap {
    pub fn parse(lines: &mut dyn Iterator<Item = Result<String, io::Error>>) -> Self {
        let mut tree_map = Self::empty();
        let mut x_index = 0;
        let mut y_index = 0;

        for line in lines {
            if let Ok(line) = line {
                for character in line.chars() {
                    let value = character.to_digit(10).unwrap();
                    tree_map.set(x_index, y_index, value);
                    y_index += 1;
                }
                y_index = 0;
            }
            x_index += 1;
        }

        tree_map
    }

    fn empty() -> Self {
        Self { map: vec![] }
    }

    fn get<'a>(&'a self, x: usize, y: usize) -> Option<&'a u32> {
        if let Some(y_vector) = self.map.get(x) {
            if let Some(value) = y_vector.get(y) {
                return Some(value);
            }
        }
        None
    }

    fn set(&mut self, x: usize, y: usize, new_value: u32) {
        if x >= self.map.len() {
            for _ in self.map.len()..x + 1 {
                self.map.push(vec![]);
            }
        }

        if let Some(y_vector) = self.map.get_mut(x) {
            if y >= y_vector.len() {
                for _ in y_vector.len()..y + 1 {
                    y_vector.push(0);
                }
            }
            if let Some(value) = y_vector.get_mut(y) {
                *value = new_value;
            }
        }
    }

    fn x_dimension(&self) -> usize {
        self.map.len()
    }

    fn y_dimension(&self) -> Option<usize> {
        if let Some(y_vector) = self.map.get(0) {
            Some(y_vector.len())
        } else {
            None
        }
    }

    pub fn how_many_trees_visible(&self) -> usize {
        let mut nb_trees_visible = 0;

        for index_x in 0..self.x_dimension() {
            for index_y in 0..self.y_dimension().unwrap() {
                if self.is_tree_visible(index_x, index_y) {
                    nb_trees_visible += 1
                }
            }
        }

        nb_trees_visible
    }

    pub fn best_position_score(&self) -> u32 {
        let mut max_score = 0;

        for index_x in 0..self.x_dimension() {
            for index_y in 0..self.y_dimension().unwrap() {
                let score = self.score(index_x, index_y);
                if score > max_score {
                    max_score = score;
                }
            }
        }

        max_score
    }

    fn score(&self, x_pos: usize, y_pos: usize) -> u32 {
        let x_dimension = self.x_dimension();
        let y_dimension = self.y_dimension().unwrap();
        let tree_height = self.get(x_pos, y_pos).unwrap();

        // Up
        let mut up_score = 0;
        for x in (0..x_pos).rev() {
            if let Some(other_tree_height) = self.get(x, y_pos) {
                up_score += 1;
                if other_tree_height >= tree_height {
                    break;
                }
            }
        }

        // Down
        let mut down_score = 0;
        for x in x_pos + 1..x_dimension {
            if let Some(other_tree_height) = self.get(x, y_pos) {
                down_score += 1;
                if other_tree_height >= tree_height {
                    break;
                }
            }
        }

        // Left
        let mut left_score = 0;
        for y in (0..y_pos).rev() {
            if let Some(other_tree_height) = self.get(x_pos, y) {
                left_score += 1;
                if other_tree_height >= tree_height {
                    break;
                }
            }
        }

        // Right
        let mut right_score = 0;
        for y in y_pos + 1..y_dimension {
            if let Some(other_tree_height) = self.get(x_pos, y) {
                right_score += 1;
                if other_tree_height >= tree_height {
                    break;
                }
            }
        }

        up_score * down_score * left_score * right_score
    }

    fn is_tree_visible(&self, x_pos: usize, y_pos: usize) -> bool {
        let x_dimension = self.x_dimension();
        let y_dimension = self.y_dimension().unwrap();

        let tree_height = self.get(x_pos, y_pos).unwrap();

        // Up
        let mut up_vibility = true;
        for x in 0..x_pos {
            if let Some(other_tree_height) = self.get(x, y_pos) {
                if other_tree_height >= tree_height {
                    up_vibility = false;
                    break;
                }
            }
        }

        // Down
        let mut down_vibility = true;
        for x in x_pos + 1..x_dimension {
            if let Some(other_tree_height) = self.get(x, y_pos) {
                if other_tree_height >= tree_height {
                    down_vibility = false;
                    break;
                }
            }
        }

        // Left
        let mut left_vibility = true;
        for y in 0..y_pos {
            if let Some(other_tree_height) = self.get(x_pos, y) {
                if other_tree_height >= tree_height {
                    left_vibility = false;
                    break;
                }
            }
        }

        // Right
        let mut right_vibility = true;
        for y in y_pos + 1..y_dimension {
            if let Some(other_tree_height) = self.get(x_pos, y) {
                if other_tree_height >= tree_height {
                    right_vibility = false;
                    break;
                }
            }
        }

        up_vibility || down_vibility || left_vibility || right_vibility
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testtools::*;

    #[test]
    fn test_parse() {
        let lines = r#""#.to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(TreeMap::parse(&mut lines).map, Vec::<Vec<u32>>::new());

        let lines = r#"3"#.to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(TreeMap::parse(&mut lines).map, [[3]]);

        let lines = r#"301
251
123"#
            .to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(
            TreeMap::parse(&mut lines).map,
            [[3, 0, 1], [2, 5, 1], [1, 2, 3]]
        );
    }

    #[test]
    fn test_how_many_trees_visible() {
        let lines = r#"30373
25512
65332
33549
35390"#
            .to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(TreeMap::parse(&mut lines).how_many_trees_visible(), 21);
    }

    #[test]
    fn test_best_position_score() {
        let lines = r#"30373
25512
65332
33549
35390"#
            .to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(TreeMap::parse(&mut lines).best_position_score(), 8);
    }
}
