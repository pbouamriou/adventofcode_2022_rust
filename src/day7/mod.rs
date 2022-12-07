use std::cell::RefCell;
use std::collections::VecDeque;
use std::io;
use std::rc::Rc;

#[derive(Debug)]
pub enum Node {
    File(String, usize),
    Dir(String, Rc<RefCell<Vec<Rc<RefCell<Node>>>>>),
}

impl Node {
    pub fn size(&self) -> usize {
        match self {
            Self::File(_, size) => *size,
            Self::Dir(_, nodes) => nodes
                .borrow()
                .iter()
                .fold(0, |accu, node| accu + node.borrow().size()),
        }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        match self {
            Self::Dir(ref name, _) => name,
            Self::File(ref name, _) => name,
        }
    }

    pub fn append_file(&mut self, file_name: &str, size: usize) -> Option<Rc<RefCell<Node>>> {
        match self {
            Self::Dir(_, childs) => {
                let found;
                {
                    let borrow_child = childs.borrow();
                    let res = borrow_child
                        .iter()
                        .find(|elem| elem.borrow().name() == file_name);
                    match res {
                        None => found = false,
                        Some(_) => found = true,
                    }
                }

                match found {
                    false => {
                        let new_nod =
                            Rc::new(RefCell::new(Node::File(file_name.to_string(), size)));
                        let mut childs = childs.borrow_mut();
                        childs.push(new_nod.clone());
                        Some(new_nod)
                    }
                    true => None,
                }
            }
            Self::File(_, _) => None,
        }
    }

    pub fn append_dir<'a>(&'a mut self, dir_name: &str) -> Option<Rc<RefCell<Node>>> {
        match self {
            Self::Dir(_, childs) => {
                let found;
                {
                    let borrow_child = childs.borrow();
                    let res = borrow_child
                        .iter()
                        .find(|elem| elem.borrow().name() == dir_name);
                    match res {
                        None => found = false,
                        Some(_) => found = true,
                    }
                }
                match found {
                    false => {
                        let new_nod = Rc::new(RefCell::new(Node::Dir(
                            dir_name.to_string(),
                            Rc::new(RefCell::new(Vec::<Rc<RefCell<Node>>>::new())),
                        )));
                        childs.borrow_mut().push(new_nod.clone());
                        Some(new_nod)
                    }
                    true => None,
                }
            }
            Self::File(_, _) => None,
        }
    }

    pub fn node_from_path<'a>(
        root_node: Rc<RefCell<Node>>,
        path: &VecDeque<String>,
    ) -> Option<Rc<RefCell<Node>>> {
        if path.len() == 0 {
            return Some(root_node);
        }
        let current_node = root_node;
        let mut path = path.clone();
        while let Some(name) = path.pop_front() {
            match &*current_node.borrow() {
                Node::Dir(_, childs) => {
                    for child in childs.borrow().iter() {
                        if child.borrow().name() == name && path.len() == 0 {
                            return Some(child.clone());
                        }
                        match &*child.borrow() {
                            Node::Dir(subdir_name, _) => {
                                if *subdir_name == name {
                                    if let Some(node) = Self::node_from_path(child.clone(), &path) {
                                        return Some(node);
                                    }
                                }
                            }
                            Node::File(_, _) => {}
                        }
                    }
                }
                Node::File(_, _) => {
                    return None;
                }
            }
        }
        None
    }
}
#[derive(PartialEq)]
enum CurrentCommand {
    Listing,
    None,
}

#[derive(Debug)]
pub struct FileSystem {
    root_node: Rc<RefCell<Node>>,
}

impl FileSystem {
    pub fn make_empty_fs() -> Self {
        Self {
            root_node: Rc::new(RefCell::new(Node::Dir(
                "/".to_string(),
                Rc::new(RefCell::new(vec![])),
            ))),
        }
    }

    pub fn make_from_listing(lines: &mut dyn Iterator<Item = Result<String, io::Error>>) -> Self {
        let mut fs = Self::make_empty_fs();

        fs.populate_from_listing(lines);

        fs
    }

    pub fn populate_from_listing(
        &mut self,
        lines: &mut dyn Iterator<Item = Result<String, io::Error>>,
    ) {
        let regex_cd = regex::Regex::new(r"^\$\s+cd\s+(?P<param>[a-zA-Z./]+)").unwrap();
        let regex_ls = regex::Regex::new(r"^\$\s+ls").unwrap();
        let regex_file = regex::Regex::new(r"(?P<size>\d+)\s+(?P<name>[a-zA-Z.]+)").unwrap();
        let regex_dir = regex::Regex::new(r"dir\s+(?P<name>[a-zA-Z.]+)").unwrap();
        let mut current_command = CurrentCommand::None;
        let mut path = VecDeque::<String>::new();
        for line in lines {
            if let Ok(line) = line {
                if let Some(captures) = regex_cd.captures(&line) {
                    // Cd command
                    match captures.name("param").unwrap().as_str() {
                        ".." => {
                            if path.len() > 0 {
                                path.pop_back();
                            }
                        }
                        "/" => path.clear(),
                        name => path.push_back(name.to_string()),
                    }
                } else if let Some(_) = regex_ls.captures(&line) {
                    current_command = CurrentCommand::Listing;
                } else if current_command == CurrentCommand::Listing {
                    if let Some(captures) = regex_file.captures(&line) {
                        let size = captures
                            .name("size")
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap();
                        let name = captures.name("name").unwrap().as_str();

                        if let Some(node) = Node::node_from_path(self.root_node.clone(), &path) {
                            node.borrow_mut().append_file(name, size);
                        }
                    } else if let Some(captures) = regex_dir.captures(&line) {
                        let name = captures.name("name").unwrap().as_str();
                        if let Some(node) = Node::node_from_path(self.root_node.clone(), &path) {
                            node.borrow_mut().append_dir(name);
                        }
                    }
                }
            }
        }
    }

    pub fn size_of_directories(&self) -> Vec<(String, usize)> {
        let mut result = vec![];
        let mut vec_dirs = vec![];
        let mut current_node = self.root_node.clone();

        {
            let current_node_borrow = current_node.borrow();
            result.push((
                current_node_borrow.name().to_string(),
                current_node_borrow.size(),
            ));
        }

        loop {
            match &*current_node.borrow() {
                Node::Dir(_, childs) => {
                    let childs_borrow = childs.borrow();
                    for child in childs_borrow.iter() {
                        match &*child.borrow() {
                            Node::Dir(subdir_name, _) => {
                                vec_dirs.push(child.clone());
                                result.push((subdir_name.clone(), child.borrow().size()));
                            }
                            Node::File(_, _) => {}
                        }
                    }
                }
                Node::File(_, _) => {}
            }
            let next_node = vec_dirs.pop();
            match next_node {
                Some(node) => current_node = node.clone(),
                None => break,
            }
        }

        result
    }

    pub fn total_size_directories(&self) -> usize {
        self.size_of_directories()
            .iter()
            .filter(|(_, size)| size <= &100000)
            .fold(0, |accu, (_, size)| accu + size)
    }

    pub fn max_directory_size(&self) -> usize {
        self.size_of_directories()
            .iter()
            .fold(0, |accu, (_, size)| if accu > *size { accu } else { *size })
    }

    pub fn directory_size_to_delete(&self) -> usize {
        let total_size = self.max_directory_size();
        let min_size_to_delete = 30000000 - (70000000 - total_size);
        self.size_of_directories()
            .iter()
            .filter(|(_, size)| *size >= min_size_to_delete)
            .fold(
                total_size,
                |accu, (_, size)| if *size < accu { *size } else { accu },
            )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testtools::*;

    #[test]
    fn test_make_from_listing() {
        let lines = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#
            .to_string();
        let mut lines = read_from_string(&lines);
        let fs = FileSystem::make_from_listing(&mut lines);
        println!("{:?}", fs);
        assert_eq!(fs.total_size_directories(), 95437);
    }
}
