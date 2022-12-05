use std::io;

pub struct StreamIterator<'a> {
    data: &'a String,
    position: usize,
}

impl<'a> Iterator for StreamIterator<'a> {
    type Item = Result<String, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lines = self.data.lines();
        let line = lines.nth(self.position);
        self.position += 1;
        match line {
            Some(str_line) => Some(Ok(str_line.to_string())),
            None => None,
        }
    }
}

pub fn read_from_string(data: &String) -> StreamIterator {
    StreamIterator { data, position: 0 }
}
