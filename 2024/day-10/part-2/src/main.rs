use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    fs,
    slice::Iter,
};

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

type Path = VecDeque<Coords>;

/// BFS used to retrived the found goal of the hiking trail
///
/// Many thanks to https://en.wikipedia.org/wiki/Breadth-first_search
fn discover_path(map: &Map, start: &Coords, already_discovered_paths: &[Path]) -> Option<Path> {
    // let Q be a queue
    let mut queue: Vec<(Path, HashSet<Coords>)> = vec![];
    let mut explored: HashSet<Coords> = HashSet::new();
    explored.insert(start.to_owned());

    queue.push((VecDeque::from(vec![start.to_owned()]), explored));
    while let Some((current_path, current_explored)) = queue.pop() {
        if map.get(&current_path[0]) == Some(&9)
            && !already_discovered_paths.contains(&current_path)
        {
            return Some(current_path);
        }

        for edge in map.get_possible_adjacent_edges(&current_path[0]) {
            if !current_explored.contains(&edge) {
                let mut new_explored = current_explored.clone();
                new_explored.insert(edge.clone());

                let mut new_path = current_path.clone();
                new_path.push_front(edge);

                queue.push((new_path, new_explored));
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
        assert_eq!(
            discover_path(&map, &Coords(3, 0), &[]),
            Some(VecDeque::from(vec![
                Coords(0, 6),
                Coords(0, 5),
                Coords(0, 4),
                Coords(0, 3),
                Coords(1, 3),
                Coords(2, 3),
                Coords(3, 3),
                Coords(3, 2),
                Coords(3, 1),
                Coords(3, 0)
            ]))
        );
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
            discover_path(&map, &Coords(3, 0), &[VecDeque::from(vec![Coords(0, 6)])]),
            Some(VecDeque::from(vec![
                Coords(6, 6),
                Coords(6, 5),
                Coords(6, 4),
                Coords(6, 3),
                Coords(5, 3),
                Coords(4, 3),
                Coords(3, 3),
                Coords(3, 2),
                Coords(3, 1),
                Coords(3, 0)
            ]))
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
            discover_path(
                &map,
                &Coords(3, 0),
                &[
                    VecDeque::from(vec![
                        Coords(0, 6),
                        Coords(0, 5),
                        Coords(0, 4),
                        Coords(0, 3),
                        Coords(1, 3),
                        Coords(2, 3),
                        Coords(3, 3),
                        Coords(3, 2),
                        Coords(3, 1),
                        Coords(3, 0)
                    ]),
                    VecDeque::from(vec![
                        Coords(6, 6),
                        Coords(6, 5),
                        Coords(6, 4),
                        Coords(6, 3),
                        Coords(5, 3),
                        Coords(4, 3),
                        Coords(3, 3),
                        Coords(3, 2),
                        Coords(3, 1),
                        Coords(3, 0)
                    ])
                ]
            ),
            None
        );
    }

    #[test]
    fn discover_trail_multiple_at_once() {
        let map = Map::from(
            "\
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
",
        );
        assert_eq!(
            discover_path(
                &map,
                &Coords(5, 0),
                &[
                    VecDeque::from(vec![
                        Coords(2, 6),
                        Coords(2, 5),
                        Coords(3, 5),
                        Coords(4, 5),
                        Coords(5, 5),
                        Coords(5, 4),
                        Coords(5, 3),
                        Coords(5, 2),
                        Coords(5, 1),
                        Coords(5, 0)
                    ]),
                    VecDeque::from(vec![
                        Coords(2, 6),
                        Coords(2, 5),
                        Coords(2, 4),
                        Coords(2, 3),
                        Coords(3, 3),
                        Coords(4, 3),
                        Coords(5, 3),
                        Coords(5, 2),
                        Coords(5, 1),
                        Coords(5, 0)
                    ]),
                    VecDeque::from(vec![
                        Coords(2, 6),
                        Coords(2, 5),
                        Coords(2, 4),
                        Coords(2, 3),
                        Coords(2, 2),
                        Coords(2, 1),
                        Coords(3, 1),
                        Coords(4, 1),
                        Coords(5, 1),
                        Coords(5, 0)
                    ])
                ]
            ),
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

    let mut paths_found = 0;
    for origin in origin_points {
        let mut already_discovered_paths: Vec<Path> = vec![];
        while let Some(path) = discover_path(map, &origin, &already_discovered_paths) {
            paths_found += 1;
            already_discovered_paths.push(path);
        }
    }
    paths_found
}

#[cfg(test)]
mod tests_count_trails {
    use super::*;

    #[test]
    fn count_trails_a() {
        let map = Map::from(
            "\
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
",
        );
        pretty_assertions::assert_eq!(count_trails(&map), 3);
    }

    #[test]
    fn count_trails_b() {
        let map = Map::from(
            "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
",
        );
        pretty_assertions::assert_eq!(count_trails(&map), 13);
    }

    #[test]
    fn count_trails_c() {
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
        pretty_assertions::assert_eq!(count_trails(&map), 81);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let map = Map::from(get_file_content("assets/input").as_str());

    println!("Result: {}", count_trails(&map));
}
