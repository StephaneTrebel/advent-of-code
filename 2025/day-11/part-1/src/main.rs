use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::Write,
    fs,
    ops::Deref,
    rc::Rc,
};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Graph<'a>(Rc<RefCell<Node<'a>>>);

impl<'a> Deref for Graph<'a> {
    type Target = Rc<RefCell<Node<'a>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node<'a> {
    name: &'a str,
    edges: Vec<Graph<'a>>,
}

impl<'a> std::fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> [{}]",
            self.name,
            self.edges.iter().fold(String::new(), |mut output, n| {
                let _ = write!(output, "{n},");
                output
            })
        )
    }
}

impl<'a> std::fmt::Display for Graph<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow())
    }
}

fn parse_content<'a>(content: &'a str) -> Graph<'a> {
    let mut node_relations_map: HashMap<&str, Vec<&str>> = HashMap::new();

    // First pass: Create node names map (their relationship on a 'string' level)
    for line in content.lines() {
        println!("line: {line}");
        let mut split = line.split(": ");
        let name = split.next().expect("Name should exist");
        println!("name: {name}");
        let neighbours_names: Vec<&str> = split
            .next()
            .expect("neighbours should exist")
            .split_whitespace()
            .collect();
        node_relations_map.insert(name, neighbours_names.clone());
    }
    println!("node_relations_map: {node_relations_map:?}");

    // Second pass: Assemble Graph, starting from "you" node
    let you = Graph(Rc::new(RefCell::new(Node {
        name: "you",
        edges: vec![],
    })));

    let mut node_map: HashMap<&str, Graph<'a>> = HashMap::new();
    node_map.insert("you", you.clone());

    let mut queue = VecDeque::new();
    queue.push_front("you".as_ref());
    while let Some(node_name) = queue.pop_back() {
        println!("Handling node name {node_name}");

        let node = node_map.get(node_name).expect("Node must exist").to_owned();

        let neighbours_names = node_relations_map
            .get(node_name)
            .expect("Node must have relationships");

        for &neighbour_name in neighbours_names {
            println!("Handling edge {neighbour_name}");
            let neighbour_node = if let Some(n) = node_map.get(neighbour_name) {
                println!("Existing node {n}");
                n.to_owned()
            } else {
                println!("New node !");
                let new_node = Graph(Rc::new(RefCell::new(Node {
                    name: neighbour_name,
                    edges: vec![],
                })));
                node_map.insert(neighbour_name, new_node.clone());
                new_node
            };

            // Only add to edges, if not already added
            if node
                .borrow()
                .edges
                .iter()
                .find(|e| e.borrow().name == neighbour_name)
                .is_none()
            {
                node.borrow_mut().edges.push(neighbour_node.clone());
            }

            queue.push_front(neighbour_name);
        }
    }

    you.clone()
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_sample() {
        let graph = parse_content(
            "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
out: 
",
        );

        let out = Graph(Rc::new(RefCell::new(Node {
            name: "out",
            edges: vec![],
        })));

        let eee = Graph(Rc::new(RefCell::new(Node {
            name: "eee",
            edges: vec![out.clone()],
        })));
        let fff = Graph(Rc::new(RefCell::new(Node {
            name: "fff",
            edges: vec![out.clone()],
        })));
        let ggg = Graph(Rc::new(RefCell::new(Node {
            name: "ggg",
            edges: vec![out.clone()],
        })));

        // let iii = Graph(Rc::new(RefCell::new(Node {
        // name: "iii",
        // edges: vec![out.clone()],
        // })));

        let ddd = Graph(Rc::new(RefCell::new(Node {
            name: "ddd",
            edges: vec![ggg.clone()],
        })));

        let ccc = Graph(Rc::new(RefCell::new(Node {
            name: "ccc",
            edges: vec![ddd.clone(), eee.clone(), fff.clone()],
        })));

        // let hhh = Graph(Rc::new(RefCell::new(Node {
        // name: "hhh",
        // edges: vec![ccc.clone()), fff.clone()), Graph(iii.clone(],
        // })));

        let bbb = Graph(Rc::new(RefCell::new(Node {
            name: "bbb",
            edges: vec![ddd.clone(), eee.clone()],
        })));

        let you = Graph(Rc::new(RefCell::new(Node {
            name: "you",
            edges: vec![bbb.clone(), ccc.clone()],
        })));

        // let aaa = Graph(Rc::new(RefCell::new(Node {
        // name: "aaa",
        // edges: vec![you.clone()), hhh.clone(],
        // })));

        assert_eq!(graph, you);
    }
}

fn fold(_graph: &Graph) -> usize {
    // graph.iter().map(Machine::decompose).sum()
    0
}

#[cfg(test)]
mod tests_fold {
    // use super::*;

    // #[test]
    // fn fold_sample() {
    // let graph = parse_content(
    // "\
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    // ",
    // );
    // assert_eq!(fold(&graph), 7);
    // }

    // #[test]
    // fn fold_final() {
    // let graph = parse_content(&get_file_content("assets/input"));
    // assert_eq!(fold(&graph), 438);
    // }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let file_content = get_file_content("assets/input");
    let graph = parse_content(&file_content);
    println!("Result: {}", fold(&graph));
}
