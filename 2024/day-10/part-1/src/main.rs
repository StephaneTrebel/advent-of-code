use std::{collections::HashSet, fmt::Display, fs, slice::Iter};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coords(isize, isize);

#[derive(Debug, PartialEq, Clone)]
struct Map(Vec<Vec<usize>>);

impl From<Vec<Vec<usize>>> for Map {
    fn from(value: Vec<Vec<usize>>) -> Self {
        Self(value)
    }
}

impl Map {
    fn get(&self, coords: &Coords) -> Option<&usize> {
        self.0
            .get(coords.1 as usize)
            .and_then(|line| line.get(coords.0 as usize))
    }

    fn iter(&self) -> Iter<'_, Vec<usize>> {
        self.0.iter()
    }

    fn get_possible_adjacent_edges(&self, position: &Coords) -> Vec<Coords> {
        let value = self.get(position).expect("Invalid position");
        let result = [
            Coords(position.0 - 1, position.1),
            Coords(position.0, position.1 - 1),
            Coords(position.0 + 1, position.1),
            Coords(position.0, position.1 + 1),
        ]
        .into_iter()
        .filter_map(|c| match self.get(&c) {
            Some(v) if (*v as isize - *value as isize) == 1 => Some(c),
            _ => None,
        })
        .collect::<Vec<Coords>>();
        result
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Self(
            value
                .split_whitespace()
                .map(|row| {
                    row.chars()
                        .map(|c| {
                            if c.is_ascii_digit() {
                                c.to_digit(10)
                                    .unwrap_or_else(|| panic!("'{}' should be a digit", &c))
                                    as usize
                            } else {
                                usize::MAX
                            }
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|row| row
                    .iter()
                    .map(|&c| {
                        if c == usize::MAX {
                            ".".to_owned()
                        } else {
                            c.to_string()
                        }
                    })
                    .collect::<String>()
                    + "\n")
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests_map {
    use super::*;

    #[test]
    fn map_from_simple() {
        let map = Map::from(
            "\
0123
1234
8765
9876
",
        );
        pretty_assertions::assert_eq!(
            map,
            Map::from(vec![
                vec![0, 1, 2, 3],
                vec![1, 2, 3, 4],
                vec![8, 7, 6, 5],
                vec![9, 8, 7, 6],
            ])
        );
        pretty_assertions::assert_eq!(
            format!("{}", map),
            "\
0123
1234
8765
9876
"
        );
    }

    #[test]
    fn map_from_less_simple() {
        let map = Map::from(
            "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
        );
        pretty_assertions::assert_eq!(
            map,
            Map::from(vec![
                vec![8, 9, 0, 1, 0, 1, 2, 3],
                vec![7, 8, 1, 2, 1, 8, 7, 4],
                vec![8, 7, 4, 3, 0, 9, 6, 5],
                vec![9, 6, 5, 4, 9, 8, 7, 4],
                vec![4, 5, 6, 7, 8, 9, 0, 3],
                vec![3, 2, 0, 1, 9, 0, 1, 2],
                vec![0, 1, 3, 2, 9, 8, 0, 1],
                vec![1, 0, 4, 5, 6, 7, 3, 2],
            ])
        );
        pretty_assertions::assert_eq!(
            format!("{}", map),
            "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
        );
    }

    #[test]
    fn map_from_impassable_terrain() {
        let map = Map::from(
            "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
",
        );
        pretty_assertions::assert_eq!(
            map,
            Map::from(vec![
                vec![
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                    0,
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                ],
                vec![
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                    1,
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                ],
                vec![
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                    2,
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                ],
                vec![6, 5, 4, 3, 4, 5, 6],
                vec![
                    7,
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                    7,
                ],
                vec![
                    8,
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                    8,
                ],
                vec![
                    9,
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                    usize::MAX,
                    9,
                ],
            ])
        );
        pretty_assertions::assert_eq!(
            format!("{}", map),
            "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
",
        );
    }
}

/// BFS used to retrived the found goal of the hiking trail
///
/// Many thanks to https://en.wikipedia.org/wiki/Breadth-first_search
fn discover_trail(map: &Map, start: &Coords, already_discovered: &[Coords]) -> Option<Coords> {
    // let Q be a queue
    let mut queue: Vec<Coords> = vec![];
    let mut explored: HashSet<Coords> = HashSet::new();
    // label start as explored
    explored.insert(start.to_owned());

    queue.push(start.to_owned());
    // while Q is not empty do
    // v := Q.dequeue()
    while let Some(v) = queue.pop() {
        // if v is an already undiscovered goal then we have discovered a hike trail
        if map.get(&v) == Some(&9) && !already_discovered.contains(&v) {
            return Some(v);
        }

        //  for all edges in G.adjacentEdges(v) do
        for edge in map.get_possible_adjacent_edges(&v) {
            // if w is not labeled as explored then
            if !explored.contains(&edge) {
                // label edge as explored
                explored.insert(edge.clone());
                // w.parent := v // Not needed here, we don't care about the actual path
                // Q.enqueue(w)
                queue.push(edge.clone());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests_discover_trail {
    use super::*;

    #[test]
    fn discover_trail_simple() {
        let map = Map::from(
            "\
...0...
...1...
...2...
6543...
7......
8......
9......
",
        );
        assert_eq!(discover_trail(&map, &Coords(3, 0), &[]), Some(Coords(0, 6)));
    }

    #[test]
    fn discover_trail_one_trail_already_discovered() {
        let map = Map::from(
            "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
",
        );
        assert_eq!(
            discover_trail(&map, &Coords(3, 0), &[Coords(0, 6)]),
            Some(Coords(6, 6))
        );
    }

    #[test]
    fn discover_trail_all_already_discovered() {
        let map = Map::from(
            "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
",
        );
        assert_eq!(
            discover_trail(&map, &Coords(3, 0), &[Coords(0, 6), Coords(6, 6)]),
            None
        );
    }
}

fn count_trails(map: &Map) -> usize {
    let origin_points: Vec<Coords> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, &c)| {
                    if c == 0 {
                        Some(Coords(x as isize, y as isize))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Coords>>()
        })
        .collect();

    let mut trails_found = 0;
    for origin in origin_points {
        let mut already_discovered: Vec<Coords> = vec![];
        while let Some(goal) = discover_trail(map, &origin, &already_discovered) {
            trails_found += 1;
            already_discovered.push(goal);
        }
    }
    trails_found
}

#[cfg(test)]
mod tests_count_trails {
    use super::*;

    #[test]
    fn count_trails_one() {
        let map = Map::from(
            "\
...0...
...1...
...2...
6543...
7......
8......
9......
",
        );
        pretty_assertions::assert_eq!(count_trails(&map), 1);
    }

    #[test]
    fn count_trails_two() {
        let map = Map::from(
            "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
",
        );
        pretty_assertions::assert_eq!(count_trails(&map), 2);
    }

    #[test]
    fn count_trails_less_simple() {
        let map = Map::from(
            "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
        );
        pretty_assertions::assert_eq!(count_trails(&map), 36);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let map = Map::from(get_file_content("assets/input").as_str());

    println!("Result: {}", count_trails(&map));
}
