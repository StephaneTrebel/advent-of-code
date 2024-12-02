use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    fs,
};
use uuid::Uuid;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

type Map = HashMap<Coords, char>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Content {
    map: Map,
    width: usize,
    height: usize,
}

fn parse_content(lines: &str) -> Content {
    Content {
        map: lines
            .split("\n")
            .filter(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| (Coords { x, y }, c))
                    .collect::<Vec<(Coords, char)>>()
            })
            .collect::<Map>(),
        width: lines.split("\n").next().unwrap().len(),
        height: lines.split("\n").filter(|line| !line.is_empty()).count(),
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            &"\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
",
        );
        assert_eq!(content.map.len(), 529);
        assert_eq!(content.width, 23);
        assert_eq!(content.height, 23);
    }
}

#[allow(dead_code)]
fn display_map(content: &Content) {
    for y in 0..content.height {
        for x in 0..content.width {
            print!("{} ", content.map.get(&Coords { x, y }).unwrap());
        }
        println!();
    }
}

type Tile = Coords;
type Node = Coords;
type Graph = BTreeMap<Node, HashMap<Node, usize>>;

#[allow(dead_code)]
fn display_graph(graph: &Graph) {
    graph.iter().for_each(|successors| {
        println!("Node: {:?}", successors.0);
        successors.1.iter().for_each(|successor| {
            println!(
                "    Successor: {:?}, Path length: {}",
                successor.0, successor.1
            )
        });
    });
}

/// Convert the map to a weighted graph
fn get_graph(content: &Content) -> Graph {
    let width = content.width;
    let height = content.height;

    let starting_point = Coords { x: 1, y: 0 };
    let finish_point = Coords {
        x: width - 2,
        y: height - 1,
    };

    // Our main datastructure
    let mut graph: Graph = Graph::new();
    // To store intermediate paths before storing them in the main graph
    let mut tmp_graph: Graph = Graph::new();
    // To ensure we only traverse the graph once
    let mut visited_tile: HashSet<Tile> = HashSet::new();
    // The iterator that will traverse the graph
    let mut queue: VecDeque<(Tile, Node)> = VecDeque::new();

    // We start by initialising a unique temp path from the origin to itself
    // Which will be expanded upon by future iterations
    tmp_graph.insert(
        starting_point.clone(),
        HashMap::from_iter(vec![(starting_point.clone(), 0)]),
    );

    // Starting point is both a tile on the map and
    // the first one that will be traversed
    visited_tile.insert(starting_point.clone());
    queue.push_back((starting_point.clone(), starting_point));

    let mut counter = 0;

    while let Some(tuple) = queue.pop_front() {
        // println!();
        // println!("Poped: {tuple:?}");
        let (current_tile, current_node) = tuple;

        // Tracking progress…
        counter += 1;
        #[cfg(test)]
        if counter > 100 {
            panic!("INFINITE LOOP DETECTED ####################################");
        }

        let x = current_tile.x;
        let y = current_tile.y;

        // Special edge case: Goal has been reached
        if current_tile == finish_point {
            graph.insert(current_tile.to_owned(), HashMap::new());
            let parent_node = tmp_graph.get_mut(&current_node).unwrap();
            let length = *parent_node.get(&current_tile).unwrap();
            match graph.get_mut(&current_node) {
                None => {
                    graph.insert(
                        current_node.to_owned(),
                        HashMap::from_iter(vec![(current_tile.to_owned(), length)]),
                    );
                }
                Some(node) => {
                    node.insert(current_tile.to_owned(), length);
                }
            }
            match graph.get_mut(&current_tile) {
                None => {
                    graph.insert(
                        current_tile.to_owned(),
                        HashMap::from_iter(vec![(current_node.to_owned(), length)]),
                    );
                }
                Some(node) => {
                    node.insert(current_node.to_owned(), length);
                }
            }
        }

        // Collect valid successors from a list of possible ones
        // A possible successor must be a traversable tile and
        // must not have been visited before
        // This will help us identify intersections (which have
        // at least two outward paths)
        let possible_successors = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(|delta| {
                let new_x = x as isize + delta.0;
                let new_y = y as isize + delta.1;
                let next_tile = Coords {
                    x: new_x as usize,
                    y: new_y as usize,
                };
                if new_x >= 0 && new_x < width as isize && new_y >= 0 && new_y < height as isize {
                    match content.map.get(&next_tile) {
                        Some('#') | None => {}
                        _ => {
                            // if let None = visited_tile.get(&next_tile) {
                            return Some((next_tile, current_node.to_owned()));
                            // }
                        }
                    }
                }
                return None;
            })
            .collect::<Vec<(Coords, Coords)>>();

        // println!("possible_successors {possible_successors:?}");

        // Valid successors are possible successors that adhere to the
        // orientation given by the tiles. Indeed Part 1 taught us that
        // the <, >, v and ^ symbols makes the input an oriented graph and
        // we can use this fact to efficiently traverse it
        let valid_successors = possible_successors
            .iter()
            .filter(|(successor, _)| match content.map.get(&successor) {
                Some(c)
                    if (*c == '<' && successor.x < x)
                        || (*c == '>' && successor.x > x)
                        || (*c == '^' && successor.y < y)
                        || (*c == 'v' && successor.y > y)
                        || (*c == '.') =>
                {
                    return true;
                }
                _ => {
                    return false;
                }
            })
            .collect::<Vec<&(Coords, Coords)>>();

        // println!("valid_successors {valid_successors:?}");

        // As said earlier possible_successors tells us if we have
        // an interserction or not
        match possible_successors.len() {
            // No intersection, thus we just follow the path
            1 | 2 => {
                // println!("Single path");
                valid_successors
                    .iter()
                    .for_each(|(next_tile, current_node)| {
                        let tmp_visited = visited_tile.get(&next_tile);
                        let tmp_node = graph.get(&next_tile);
                        // If next_tile is a known node, we need to visit it again
                        // to close the current path, but if it's not a node we
                        // just need to ensure it has not been already visited
                        if (next_tile != current_node && tmp_node.is_some())
                            || tmp_visited.is_none()
                        {
                            // Now we can mark the successor as visited since we are now
                            // sure to use it
                            visited_tile.insert(next_tile.to_owned());
                            // Increment path length on our temporary path map
                            let node = tmp_graph.get_mut(&current_node).unwrap();
                            // println!("current_tile {current_tile:?} node {node:?}");
                            let previous_length = node.get(&current_tile).unwrap();
                            node.insert(next_tile.to_owned(), previous_length + 1);

                            queue.push_back((next_tile.to_owned(), current_node.to_owned()));
                        }
                    })
            }
            // Intersections handling
            e if e > 2 => {
                // println!("Intersection");

                // Since we arrived at an intersection, we have to "promote" our
                // temporary path to a double sided one, to later be able to
                // traverse our graph through both ways
                let parent_node = tmp_graph.get(&current_node).unwrap();
                let current_length = parent_node.get(&current_tile).unwrap();
                match graph.get_mut(&current_node) {
                    None => {
                        // println!("Creating new node {current_node:?} and adding it a path of length {current_length} to {current_tile:?}");
                        graph.insert(
                            current_node.to_owned(),
                            HashMap::from_iter(vec![(current_tile.to_owned(), *current_length)]),
                        );
                    }
                    Some(node) => {
                        // println!("Adding path from node {current_node:?} to {current_tile:?} of length {current_length}");
                        node.insert(current_tile.to_owned(), *current_length);
                    }
                }
                match graph.get_mut(&current_tile) {
                    None => {
                        // println!("Creating new node {current_tile:?} and adding it a path of length {current_length} to {current_node:?}");
                        graph.insert(
                            current_tile.to_owned(),
                            HashMap::from_iter(vec![(current_node.to_owned(), *current_length)]),
                        );
                        tmp_graph.insert(
                            current_tile.to_owned(),
                            HashMap::from_iter(vec![(current_node.to_owned(), *current_length)]),
                        );
                    }
                    Some(node) => {
                        // println!("Adding path from node {current_node:?} to {current_tile:?} of length {current_length}");
                        node.insert(current_node.to_owned(), *current_length);
                    }
                }

                // Initialize temporary paths for this new node
                // but only if the node is not already known
                // let maybe_current_node = tmp_graph.get(&current_tile);
                // if maybe_current_node.is_none() {
                // graph.insert(current_tile.to_owned(), HashMap::new());
                // tmp_graph.insert(current_tile.to_owned(), HashMap::new());
                // }

                // Create new temporary paths for each branch
                valid_successors.iter().for_each(|(next_tile, _)| {
                    // println!("New intersection: {current_tile:?}");
                    let node = tmp_graph.get_mut(&current_tile).unwrap();
                    // println!("Adding successor {next_tile:?} to intersection");
                    node.insert(next_tile.to_owned(), 1);
                    queue.push_back((next_tile.to_owned(), current_tile.to_owned()));
                });
            }
            _ => {
                // println!("No valid successors");
            }
        }
    }

    println!("Took {counter} steps");

    graph
}

#[cfg(test)]
mod tests_get_graph {
    use super::*;

    #[test]
    fn get_graph_01() {
        let content = parse_content(
            &"\
#.##
#..#
##.#
",
        );

        let result = get_graph(&content);

        assert_eq!(
            result,
            BTreeMap::from_iter(vec![
                (
                    Coords { x: 1, y: 0 },
                    HashMap::from_iter(vec![(Coords { x: 2, y: 2 }, 3)])
                ),
                (
                    Coords { x: 2, y: 2 },
                    HashMap::from_iter(vec![(Coords { x: 1, y: 0 }, 3)])
                ),
            ])
        );
    }

    #[test]
    fn get_graph_test_input() {
        let content = parse_content(
            &"\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
",
        );

        let graph = get_graph(&content);

        display_graph(&graph);

        assert_eq!(
            graph,
            BTreeMap::from_iter(vec![
                (
                    Coords { x: 1, y: 0 },
                    HashMap::from_iter(vec![(Coords { x: 3, y: 5 }, 15)]),
                ),
                (
                    Coords { x: 3, y: 5 },
                    HashMap::from_iter(vec![
                        (Coords { x: 1, y: 0 }, 15),
                        (Coords { x: 5, y: 13 }, 22),
                        (Coords { x: 11, y: 3 }, 22),
                    ]),
                ),
                (
                    Coords { x: 5, y: 13 },
                    HashMap::from_iter(vec![
                        (Coords { x: 3, y: 5 }, 22),
                        (Coords { x: 13, y: 13 }, 12),
                        (Coords { x: 13, y: 19 }, 38),
                    ]),
                ),
                (
                    Coords { x: 11, y: 3 },
                    HashMap::from_iter(vec![
                        (Coords { x: 21, y: 11 }, 30),
                        (Coords { x: 13, y: 13 }, 24),
                        (Coords { x: 3, y: 5 }, 22),
                    ]),
                ),
                (
                    Coords { x: 13, y: 13 },
                    HashMap::from_iter(vec![
                        (Coords { x: 13, y: 19 }, 10),
                        (Coords { x: 11, y: 3 }, 24),
                        (Coords { x: 5, y: 13 }, 12),
                        // (Coords { x: 21, y: 11 }, 18),
                    ]),
                ),
                (
                    Coords { x: 13, y: 19 },
                    HashMap::from_iter(vec![
                        (Coords { x: 13, y: 13 }, 10),
                        (Coords { x: 5, y: 13 }, 38),
                        (Coords { x: 19, y: 19 }, 10),
                    ]),
                ),
                (
                    Coords { x: 19, y: 19 },
                    HashMap::from_iter(vec![
                        (Coords { x: 21, y: 22 }, 5),
                        (Coords { x: 21, y: 11 }, 10),
                        (Coords { x: 13, y: 19 }, 10),
                    ]),
                ),
                (
                    Coords { x: 21, y: 11 },
                    HashMap::from_iter(vec![
                        (Coords { x: 19, y: 19 }, 10),
                        (Coords { x: 11, y: 3 }, 30),
                        // (Coords { x: 13, y: 13 }, 18),
                    ]),
                ),
                (
                    Coords { x: 21, y: 22 },
                    HashMap::from_iter(vec![(Coords { x: 19, y: 19 }, 5)]),
                ),
            ])
        );
    }
}

fn find_longest_path(graph: &Graph, starting_node: &Node, finish_node: &Node) -> usize {
    let starting_uuid = Uuid::new_v4();

    // Visited is a HashMap whose key cannot be only Coords since we want the
    // longest path.
    // So, we have to track both a tile coordinates and the path (UUID) it's on
    // Parent is optional because the starting point does not have a parent.
    // Also, a "Parent" is a couple of Coords and usize because a parent will
    // not be on the same path (at each intersection we create new paths)
    let mut visited: HashMap<(Node, Uuid), Option<Node>> = HashMap::new();

    // Ultimately we will only need one path_id: the one that is the longest
    let mut longest_path_length = 0;

    visited.insert((starting_node.clone(), starting_uuid), None);

    // Path id is stored in the queue to match a successor with the according visited map
    let mut queue: VecDeque<(Node, Uuid)> =
        VecDeque::from_iter(vec![(starting_node.to_owned(), starting_uuid)]);

    let mut counter = 0;

    while let Some(element) = queue.pop_back() {
        // println!();
        // println!("Poped: {element:?}");
        // println!(
        // "Queue elements remaining: {:?}, Visited size: {:?}",
        // queue.len(),
        // visited.len()
        // );
        #[cfg(test)]
        if counter > 1000 {
            panic!("INFINITE LOOP DETECTED ####################################");
        }

        // Tracking progress…
        counter += 1;
        // if counter % 100 == 0 {
        // println!("{counter}");
        // }

        let current_node = element.0;
        let current_path = element.1;

        // Goal
        if current_node == *finish_node {
            // Processing the path to determine if it's better than the best existing one
            let mut length = 0;
            let mut tmp_element = current_node.clone();
            // println!("visited {visited:?}");
            // println!("current_path {current_path:?}");
            // println!("tmp_element {tmp_element:?}");
            while let Some(ancestor_tuple) = visited
                .clone()
                .iter()
                .find(|(k, s)| match s {
                    None => false,
                    Some(e) => k.1 == current_path && *e == tmp_element,
                })
                .map(|(parent, _)| parent)
            {
                let found_length = graph
                    .get(&ancestor_tuple.0)
                    .unwrap()
                    .get(&tmp_element)
                    .unwrap();
                // println!("ancestor_tuple: {ancestor_tuple:?} found_length: {found_length}");
                length += found_length;
                tmp_element = ancestor_tuple.0.clone();
            }
            if length > longest_path_length {
                println!("longest_path_length CHANGED TO {length}");
                longest_path_length = length;
            }

            // Discarding path to lighten the map
            let to_purge: Vec<(Node, Uuid)> = visited
                .keys()
                .filter(|k| k.1 == current_path)
                .map(|k| k.to_owned())
                .collect::<Vec<(Node, Uuid)>>();
            // println!("Purging {} elements", to_purge.len());
            for element in to_purge {
                visited.remove(&element);
            }

            // println!("Continuing…");
            continue;
        }

        // println!("Node {current_node:?} Path {current_path}");
        // Collect valid successors from a list of possible ones
        let valid_successors = graph
            .iter()
            .filter(|(node, _)| **node == current_node)
            .flat_map(|(node, successors)| {
                successors.iter().map(|s| {
                    if let None = visited.get(&(s.0.to_owned(), current_path)) {
                        // println!("successor {successor:?} is valid");
                        return Some((
                            s.0.to_owned(),
                            node.to_owned(),
                            s.1.to_owned(),
                            current_path,
                        ));
                    }
                    return None;
                })
            })
            .filter_map(|e| e)
            .collect::<Vec<(Node, Node, usize, Uuid)>>();

        // println!("valid_successors {valid_successors:?}");

        // If we are at an intersection
        if valid_successors.len() > 1 {
            // println!("Intersection: {current_node:?}, {current_path}");
            valid_successors
                .iter()
                .for_each(|(successor, parent, _, _)| {
                    // println!("Successor: {successor:?}, Parent: {parent:?}");
                    // Uuid to ensure path id uniqueness
                    let new_path_id = Uuid::new_v4();

                    // Algorithm core: we duplicate the whole parent chain
                    // for the new generated path, to ensure that we won't visit
                    // tiles in the future

                    let mut tmp_element = parent.clone();
                    while let Some(ancestor_tuple) =
                        // visited.get(&(tmp_element.clone(), parent_path_id.clone()))
                        visited
                            .clone()
                            .iter()
                            .find(|(k, s)| match s {
                                None => false,
                                Some(e) => *e == tmp_element && k.1 == current_path,
                            })
                            .map(|(parent, _)| parent)
                    {
                        // println!("ancestor_tuple: {ancestor_tuple:?}");
                        visited.insert((ancestor_tuple.0.clone(), new_path_id), Some(tmp_element));
                        tmp_element = ancestor_tuple.0.clone();
                    }

                    // We now can consider the successor visited on its new path
                    visited.insert((parent.clone(), new_path_id), Some(successor.clone()));
                    // And we iterate on it !
                    queue.push_back((successor.clone(), new_path_id));
                })
        }
        // If not an intersection, we go further in the path
        else {
            valid_successors
                .iter()
                .for_each(|(successor, parent, _, path_id)| {
                    // println!("Node: {current_node:?}, {current_path}, {successor:?}");
                    visited.insert((parent.to_owned(), *path_id), Some(successor.to_owned()));
                    queue.push_back((successor.to_owned(), *path_id));
                })
        }

        // if there are no more queue element for the element path, we purge the visited map
        // to improve performance
        if queue.iter().find(|e| e.1 == current_path).is_none() {
            let to_purge: Vec<(Node, Uuid)> = visited
                .keys()
                .filter(|k| k.1 == current_path)
                .map(|k| k.to_owned())
                .collect::<Vec<(Node, Uuid)>>();
            // println!("Purging {} elements", to_purge.len());
            for element in to_purge {
                visited.remove(&element);
            }
        }
    }

    println!("Took {counter} steps");

    longest_path_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_longest_path_01() {
        let graph = BTreeMap::from_iter(vec![
            (
                Coords { x: 1, y: 0 },
                HashMap::from_iter(vec![(Coords { x: 3, y: 5 }, 15)]),
            ),
            (
                Coords { x: 3, y: 5 },
                HashMap::from_iter(vec![
                    (Coords { x: 11, y: 3 }, 22),
                    (Coords { x: 5, y: 13 }, 22),
                    (Coords { x: 1, y: 0 }, 15),
                ]),
            ),
            (
                Coords { x: 5, y: 13 },
                HashMap::from_iter(vec![
                    (Coords { x: 13, y: 13 }, 12),
                    (Coords { x: 3, y: 5 }, 22),
                    (Coords { x: 13, y: 19 }, 38),
                ]),
            ),
            (
                Coords { x: 11, y: 3 },
                HashMap::from_iter(vec![
                    (Coords { x: 21, y: 11 }, 30),
                    (Coords { x: 13, y: 13 }, 24),
                    (Coords { x: 3, y: 5 }, 22),
                ]),
            ),
            (
                Coords { x: 13, y: 13 },
                HashMap::from_iter(vec![
                    (Coords { x: 5, y: 13 }, 12),
                    (Coords { x: 11, y: 3 }, 24),
                    (Coords { x: 13, y: 19 }, 10),
                    // Not necessary (Coords { x: 21, y: 11 }, 18),
                ]),
            ),
            (
                Coords { x: 13, y: 19 },
                HashMap::from_iter(vec![
                    (Coords { x: 5, y: 13 }, 38),
                    (Coords { x: 13, y: 13 }, 10),
                    (Coords { x: 19, y: 19 }, 10),
                ]),
            ),
            (
                Coords { x: 21, y: 11 },
                HashMap::from_iter(vec![
                    (Coords { x: 11, y: 3 }, 30),
                    // Not necessary (Coords { x: 13, y: 13 }, 18),
                    (Coords { x: 19, y: 19 }, 10),
                ]),
            ),
            (
                Coords { x: 19, y: 19 },
                HashMap::from_iter(vec![
                    (Coords { x: 21, y: 22 }, 5),
                    (Coords { x: 21, y: 11 }, 10),
                    (Coords { x: 13, y: 19 }, 10),
                ]),
            ),
            (
                Coords { x: 21, y: 22 },
                HashMap::from_iter(vec![(Coords { x: 19, y: 19 }, 5)]),
            ),
        ]);

        let result = find_longest_path(&graph, &Coords { x: 1, y: 0 }, &Coords { x: 21, y: 22 });

        assert_eq!(result, 153);
    }

    #[test]
    fn find_longest_path_main() {
        let content = &parse_content(&get_file_content("assets/input"));
        println!("Converting to graph…");
        let graph = get_graph(&content);
        println!("Done");

        // @TODO: The Algorithm will kinda quickly give the right value for my input
        // So I kinda cheated and went with it. So what's left here is optimize the F
        // out of it and _at LEAST_ make it finish
        // Several leads to do that (courtesy of https://old.reddit.com/r/adventofcode/comments/192oi3p/2023_day_23_part_2_c_help_me_make_my_code_faster/):
        // - Don't start and end on the start/end tile, since they are on isolated paths we already know about
        // - Don't use HashMap for visited. Use a [Bitmap](https://docs.rs/bitmap/latest/bitmap/struct.Bitmap.html) instead
        // - Parrelize by DFSing a list of starting position and hammer everything at once with Rayen
        // - Use heuristics ! Comparing current_path with already determined path length can help
        // forgoing paths that we know for sure won't be able to beat the longest one
        assert_eq!(
            find_longest_path(&graph, &Coords { x: 1, y: 0 }, &Coords { x: 139, y: 140 }),
            9999
        );
    }
}

fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Converting to graph…");
    let graph = get_graph(&content);
    println!("Done");

    println!(
        "Part 2: {}",
        find_longest_path(&graph, &Coords { x: 1, y: 0 }, &Coords { x: 139, y: 140 })
    );
}
