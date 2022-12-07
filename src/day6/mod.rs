use std::io;

pub fn find_start_of_packet(
    lines: &mut dyn Iterator<Item = Result<String, io::Error>>,
) -> Option<usize> {
    find_start_of_sequence(lines, 4)
}

pub fn find_start_of_message(
    lines: &mut dyn Iterator<Item = Result<String, io::Error>>,
) -> Option<usize> {
    find_start_of_sequence(lines, 14)
}

fn find_start_of_sequence(
    lines: &mut dyn Iterator<Item = Result<String, io::Error>>,
    sequence_size: usize,
) -> Option<usize> {
    let line = lines.next();
    if let Some(Ok(message)) = line {
        let message_len = message.len();
        for position in 0..message_len - sequence_size {
            let subpacket = &message[position..position + sequence_size];
            let mut good_start = true;
            'pattern_search: for start in 0..sequence_size - 1 {
                for index in start + 1..sequence_size {
                    if subpacket.chars().nth(index) == subpacket.chars().nth(start) {
                        good_start = false;
                        break 'pattern_search;
                    }
                }
            }

            if good_start {
                return Some(position + sequence_size);
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use std::io::Lines;

    use super::*;
    use crate::testtools::*;

    #[test]
    fn test_find_start_of_packet() {
        let lines = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#.to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(find_start_of_packet(&mut lines), Some(7));

        let lines = r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#.to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(find_start_of_packet(&mut lines), Some(5));

        let lines = r#"nppdvjthqldpwncqszvftbrmjlhg"#.to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(find_start_of_packet(&mut lines), Some(6));

        let lines = r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#.to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(find_start_of_packet(&mut lines), Some(10));

        let lines = r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#.to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(find_start_of_packet(&mut lines), Some(11));
    }

    #[test]
    fn test_find_start_of_message() {
        let lines = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#.to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(find_start_of_message(&mut lines), Some(19));

        let lines = r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#.to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(find_start_of_message(&mut lines), Some(23));

        let lines = r#"nppdvjthqldpwncqszvftbrmjlhg"#.to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(find_start_of_message(&mut lines), Some(23));

        let lines = r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#.to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(find_start_of_message(&mut lines), Some(29));

        let lines = r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#.to_string();
        let mut lines = read_from_string(&lines);
        assert_eq!(find_start_of_message(&mut lines), Some(26));
    }
}
