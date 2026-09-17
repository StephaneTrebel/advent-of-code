use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn parse_content(content: &str) -> HashMap<&str, Vec<&str>> {
    println!("Parsing graph...");
    let mut node_relations_map: HashMap<&str, Vec<&str>> = HashMap::new();

    // First pass: Create node names map (their relationship on a 'string' level)
    for line in content.lines() {
        let mut split = line.split(": ");
        let name = split.next().expect("Name should exist");
        let neighbours_names: Vec<&str> = split
            .next()
            .expect("neighbours should exist")
            .split_whitespace()
            .collect();
        node_relations_map.insert(name, neighbours_names.clone());
    }
    // println!("node_relations_map: {node_relations_map:?}");

    node_relations_map
}

fn fold(graph: &HashMap<&str, Vec<&str>>, start: &str, finish: &str) -> u64 {
    println!("Folding graph from {start} to {finish}");
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut dead_branch_list: HashSet<&str> = HashSet::new();
    let mut iteration_count = 0;
    queue.push_front(start);

    // let mut paths = u64::from(finish == "out");
    let mut paths = 1;
    while let Some(node) = queue.pop_front() {
        iteration_count += 1;
        if iteration_count % 10000 == 0 {
            // println!("Iteration {iteration_count}…");
        }
        // println!(
        // "ℹ️ New entry: {node}, Count: {iteration_count}, dead_branch_list length: {}, paths: {paths}",
        // dead_branch_list.len()
        // );
        let children = graph.get(node).expect("Children should exist");
        let children_count = children.len();
        // println!("Children count {children_count}");
        if children_count > 1 {
            paths += u64::try_from(children_count - 1).expect("Berk !");
        }

        if children
            .iter()
            .all(|child| dead_branch_list.contains(child))
        {
            // println!("All children are on a dead branch, add node to dead_branch_list");
            dead_branch_list.insert(node);
        }

        for &child in children {
            // println!("Current child {child}");
            // PART-1 --------------------------------------------------------------
            // When the graph browsing CANNOT have "dead branches", like part1
            // or "dac->out"
            if finish == "out" {
                if child == finish {
                    // println!("🎉Reached FINISH node !!!");
                } else {
                    // If we have children to browse, we add them to the queue
                    // println!("Inserting new child in queue {child}");
                    queue.push_front(child);
                }
            }
            // PART-2 --------------------------------------------------------------
            // When the graph browsing can have "dead branches", which are branches
            // that never lead to our finish node (like in part2 with svr->fft)
            else {
                // Optimization: we do not need to browse a branch if we already know
                // it is a dead one
                if dead_branch_list.contains(node) {
                    // println!("⚠️ node {node} is already in dead branch list");
                    paths -= 1;
                    continue;
                }
                // Have we reached a "dead branch" finish node, meaning we don't want
                // to count this path anymore !
                if child == "out" {
                    // println!("✅ Reaching dead branch finish node for node {node}");
                    paths -= 1;
                    dead_branch_list.insert(node);
                }
                // If we have reached the finish node,
                // the path is already accounted for, so we do nothing
                else if child == finish {
                    // println!("🎉 Reached FINISH node !!!");
                }
                // Else we have children to browse, so we add them to the queue
                else {
                    // else if !queue.contains(&child) {
                    // println!("Inserting new child in queue {child}");
                    queue.push_front(child);
                }
                // else {
                // println!("Child {child} already in queue");
                // }
            }
        }

        // println!("New path count: {paths}");
    }

    // println!("Dead branch list: {dead_branch_list:?}, paths: {paths}\n");
    paths
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_sample_part1() {
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
",
        );

        assert_eq!(fold(&graph, "you", "out"), 5);
    }

    #[test]
    fn fold_sample_part2_sample() {
        let graph = parse_content(
            "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
",
        );

        assert_eq!(fold(&graph, "svr", "fft"), 1);
        assert_eq!(fold(&graph, "fft", "dac"), 1);
        assert_eq!(fold(&graph, "dac", "out"), 2);
    }

    #[test]
    fn fold_sample_part2_real() {
        let file_content = get_file_content("assets/input");
        let graph = parse_content(&file_content);

        assert_eq!(fold(&graph, "svr", "fft"), 7394);
        assert_eq!(fold(&graph, "fft", "dac"), 13_165_590);
        assert_eq!(fold(&graph, "dac", "out"), 3806);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let file_content = get_file_content("assets/input");
    let graph = parse_content(&file_content);
    let svr_fft = fold(&graph, "svr", "fft");
    println!("Result svr->fft: {svr_fft}");
    let fft_dac = fold(&graph, "fft", "dac");
    println!("Result fft->dac: {fft_dac}");
    let dac_out = fold(&graph, "dac", "out");
    println!("Result dac->out: {dac_out}");
    println!("Product: {}", svr_fft * fft_dac * dac_out); // Product: 370500293582760
}
