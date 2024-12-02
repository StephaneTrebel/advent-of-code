use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

use rand::Rng;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

type Nodes = Vec<String>;
type Graph = HashMap<String, Nodes>;

#[derive(Debug, PartialEq, Clone)]
struct Content {
    graph: Graph,
}

fn parse_content(lines: &str) -> Content {
    Content {
        graph: lines
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut split = line.split(":");
                (
                    split.next().unwrap().to_owned(),
                    split
                        .next()
                        .unwrap()
                        .split_ascii_whitespace()
                        .map(|e| e.replace(" ", ""))
                        .collect(),
                )
            })
            .collect::<Graph>(),
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            &"\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
",
        );
        assert_eq!(content.graph.get("bvb").unwrap(), &vec!["xhk", "hfx"]);
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
struct Node {
    count: usize,
}

// Using Karger's algorithm to retrieve the min_cut and, most importantly,
// the multplication of both remaining graph node count (which is the wanted answer)
//
// See https://en.wikipedia.org/wiki/Karger%27s_algorithm for more information
fn get_min_cut(graph: &Graph) -> u32 {
    println!("############################### Start");

    let mut vertices: HashMap<String, Node> = HashMap::new();
    let mut edges: Vec<(String, String)> = vec![];
    let mut rng = rand::thread_rng();

    graph.iter().for_each(|(k, v_list)| {
        vertices.insert(k.to_owned(), Node { count: 1 });
        v_list.iter().for_each(|v| {
            if vertices.get(v).is_none() {
                vertices.insert(v.to_owned(), Node { count: 1 });
            }
            edges.push((
                min(k.to_owned(), v.to_owned()),
                max(k.to_owned(), v.to_owned()),
            ));
        })
    });

    let result: u32;
    // Many thanks to https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day25/src/main.rs
    // for the «last mile» regarding "edges" switching !
    loop {
        let mut tmp_edges: Vec<(String, String)> = edges.clone();
        let mut vertices_count = vertices.len();
        for vertex in &mut vertices {
            vertex.1.count = 1;
        }
        while vertices_count > 2 {
            // Choose two vertices at random and contract them
            // let mut temp_vertices = vertices.clone();
            let (a, b): (String, String) = tmp_edges.swap_remove(rng.gen_range(0..tmp_edges.len()));
            vertices.get_mut(&a).unwrap().count += vertices.get(&b).unwrap().count;

            let mut i = 0;
            while i < tmp_edges.len() {
                let (c, d) = tmp_edges[i].clone();
                if (&c, &d) == (&a, &b) {
                    tmp_edges.swap_remove(i);
                } else if c == b {
                    tmp_edges[i] = (
                        min(a.to_owned(), d.to_owned()),
                        max(a.to_owned(), d.to_owned()),
                    );
                    i += 1;
                } else if d == b {
                    tmp_edges[i] = (
                        min(a.to_owned(), c.to_owned()),
                        max(a.to_owned(), c.to_owned()),
                    );
                    i += 1;
                } else {
                    i += 1;
                }
            }
            vertices_count -= 1;
        }

        if tmp_edges.len() == 3 {
            let (a, b) = tmp_edges[0].to_owned();
            result = vertices[&a].count as u32 * vertices[&b].count as u32;
            break;
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_min_cut_test_input() {
        let content = parse_content(
            &"\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
",
        );
        assert_eq!(get_min_cut(&content.graph), 54);
    }
}

fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Part 1: {}", get_min_cut(&content.graph));
}
