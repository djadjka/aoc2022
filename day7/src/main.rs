use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

static FILE_PATH: &str = "./day7/input";

#[derive(Clone, Debug)]
struct Node {
    name: String,
    size: usize,
    children: Vec<usize>,
    parent: Option<usize>,
}

impl Node {
    fn add_child(&mut self, child_index: usize) {
        self.children.push(child_index);
    }

    fn increase_size(&mut self, size: usize) {
        self.size += size;
    }

    fn new(name: String, parent: Option<usize>, size: usize) -> Node {
        Node {
            children: Vec::new(),
            parent,
            name,
            size,
        }
    }

    fn is_file(&self) -> bool {
        self.children.is_empty()
    }

    fn clean_size(&mut self) {
        self.size = 0;
    }
}

#[derive(Clone, Debug)]
struct FileSystem {
    nodes: Vec<Node>,
    cur_node: usize,
}

impl FileSystem {
    fn cd_up(&mut self) {
        self.cur_node = self.nodes[self.cur_node]
            .parent
            .expect("parrent must exist");
    }

    fn cd_down(&mut self, name: String) {
        for node_index in self.nodes[self.cur_node].children.iter() {
            if self.nodes[*node_index].name == name {
                self.cur_node = *node_index;
                return;
            }
        }
    }

    fn dir(&mut self, name: String) {
        self.file(name, 0);
    }

    fn file(&mut self, name: String, size: usize) {
        let new_node = Node::new(name, Some(self.cur_node), size);
        self.nodes.push(new_node);
        let index = self.nodes.len() - 1;
        self.nodes[self.cur_node].add_child(index);
    }

    fn init() -> FileSystem {
        FileSystem {
            nodes: vec![Node::new("/".to_string(), None, 0)],
            cur_node: 0,
        }
    }
}

fn parse_comand(comand: String, file_system: &mut FileSystem) {
    let splited_comand: Vec<&str> = comand.split_whitespace().collect();
    if splited_comand[0] == "$" {
        match splited_comand[1] {
            "cd" => {
                if splited_comand[2] == ".." {
                    file_system.cd_up();
                } else {
                    let name = splited_comand[2];
                    file_system.cd_down(name.to_string());
                }
            }
            "ls" => {}
            _ => panic!("unattainable"),
        }
    } else {
        match splited_comand[0] {
            "dir" => file_system.dir(splited_comand[1].to_string()),
            _ => file_system.file(
                splited_comand[1].to_string(),
                splited_comand[0]
                    .parse::<usize>()
                    .expect("must be a number"),
            ),
        }
    }
}

fn count_sizes(nodes: &mut Vec<Node>, index: usize, answer: &mut usize) -> usize {
    if !nodes[index].is_file() {
        nodes[index].clean_size();
    }
    
    for i in nodes[index].children.clone().iter() {
        let size = count_sizes(nodes, *i, answer);
        nodes[index].increase_size(size);
    }

    if !nodes[index].is_file() && nodes[index].size <= 100000 {
        *answer += nodes[index].size;
    }

    nodes[index].size
}

fn first(file_system: FileSystem) -> usize {
    let mut answer = 0;
    let mut nodes = file_system.nodes;
    count_sizes(&mut nodes, 0, &mut answer);
    answer
}

fn second(file_system: FileSystem) -> usize {
    let mut nodes = file_system.nodes;
    let mut temp = 0;
    count_sizes(&mut nodes, 0, &mut temp);

    let need_space = 30000000 - (70000000 - nodes[0].size);

    let folder_nodes: Vec<&Node> = nodes
        .iter()
        .filter(|n| !n.is_file() && n.size >= need_space)
        .collect();

    let mut min = folder_nodes[0].size;

    for folder_node in folder_nodes[1..].iter() {
        if folder_node.size < min {
            min = folder_node.size;
        }
    }
    min
}

fn read_file_system() -> Result<FileSystem, io::Error> {
    let f = File::open(FILE_PATH)?;
    let reader = BufReader::new(f);
    let mut file_system = FileSystem::init();
    for l in reader.lines() {
        let line = l?;
        parse_comand(line, &mut file_system);
    }
    Ok(file_system)
}

fn main() {
    match read_file_system() {
        Ok(file_system) => {
            println!("first: {}", first(file_system.clone()));
            println!("second: {}", second(file_system));
        }
        Err(e) => panic!("{}", e),
    }
}
