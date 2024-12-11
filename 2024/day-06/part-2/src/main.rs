use std::{collections::HashSet, fs};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

type Map = Vec<Vec<char>>;
#[derive(Debug, PartialEq, Clone)]
struct Content {
    map: Map,
}

#[allow(dead_code)]
fn display_map(map: &Map) {
    map.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!()
    });
}

fn parse_content(input: &str) -> Content {
    Content {
        map: input
            .split_whitespace()
            .map(|line| line.chars().collect())
            .collect(),
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
        );
        assert_eq!(
            content,
            Content {
                map: vec![
                    vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
                    vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.']
                ]
            }
        );
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

fn turn_right(orientation: &Orientation) -> Orientation {
    match orientation {
        Orientation::Up => Orientation::Right,
        Orientation::Down => Orientation::Left,
        Orientation::Left => Orientation::Up,
        Orientation::Right => Orientation::Down,
    }
}

enum Way {
    Forward,
    Backward,
}

fn displace(
    (x, y): (usize, usize),
    orientation: &Orientation,
    (max_x, max_y): (usize, usize),
    way: Way,
) -> Option<(usize, usize)> {
    match orientation {
        Orientation::Up => match way {
            Way::Forward => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Way::Backward => {
                if y < max_y - 1 {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
        },
        Orientation::Down => match way {
            Way::Forward => {
                if y < max_y - 1 {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
            Way::Backward => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
        },
        Orientation::Right => match way {
            Way::Forward => {
                if x < max_x - 1 {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
            Way::Backward => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
        },
        Orientation::Left => match way {
            Way::Forward => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Way::Backward => {
                if x < max_x - 1 {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
        },
    }
}

/// Walk the gard through the Map
fn ends_up_in_a_loop(map: &Map, (initial_x, initial_y): (usize, usize)) -> bool {
    let (max_x, max_y) = (map[0].len(), map.len());
    let mut updated_map = map.clone();
    let mut orientation = Orientation::Up;
    let mut position = Some((initial_x, initial_y));
    let mut trace: HashSet<((usize, usize), Orientation)> = HashSet::new();

    while let Some((x, y)) = position {
        if trace.contains(&((x, y), orientation.clone())) {
            return true;
        }
        trace.insert(((x, y), orientation.clone()));
        let mut new_orientiation = orientation.clone();
        match updated_map.get(y).map(|line| line.get(x)) {
            Some(Some('#')) | Some(Some('O')) => {
                position = displace((x, y), &orientation, (max_x, max_y), Way::Backward);
                new_orientiation = turn_right(&new_orientiation);
            }
            _ => {
                updated_map[y][x] = 'X';
            }
        }
        if let Some((x, y)) = position {
            position = displace((x, y), &new_orientiation, (max_x, max_y), Way::Forward);
        }
        orientation = new_orientiation.clone();
    }

    false
}

#[cfg(test)]
mod tests_walk_the_guard {
    use super::*;

    #[test]
    fn walk_the_guard_01() {
        let content = parse_content(
            "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#.O^.....
........#.
#.........
......#...
",
        );
        assert!(ends_up_in_a_loop(&content.map, (4, 6)));
    }
}

/// How many positions can be used to stuck the guard in a loop ?
fn fold(map: &Map, starting_position: (usize, usize)) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().map(move |(x, _)| {
                let mut map_with_obstacle = map.clone();
                map_with_obstacle[y][x] = 'O';
                ends_up_in_a_loop(&map_with_obstacle, starting_position)
            })
        })
        .filter(|&b| b)
        .count()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_01() {
        let content = parse_content(
            "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
        );
        assert_eq!(fold(&content.map, (4, 6)), 6);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = &mut parse_content(&get_file_content("assets/input"));
    let starting_position = (48, 71);

    println!("Result: {}", fold(&content.map, starting_position));
}
