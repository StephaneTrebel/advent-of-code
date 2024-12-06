use std::fs;

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
fn walk_the_guard(map: &Map, (initial_x, initial_y): (usize, usize)) -> Map {
    let (max_x, max_y) = (map[0].len(), map.len());
    let mut updated_map = map.clone();
    let mut orientation = Orientation::Up;
    let mut position = Some((initial_x, initial_y));

    while let Some((x, y)) = position {
        match updated_map.get(y).map(|line| line.get(x)) {
            Some(Some('#')) => {
                position = displace((x, y), &orientation, (max_x, max_y), Way::Backward);
                orientation = turn_right(&orientation);
            }
            _ => {
                updated_map[y][x] = 'X';
            }
        }
        if let Some((x, y)) = position {
            position = displace((x, y), &orientation, (max_x, max_y), Way::Forward);
        }
    }

    updated_map
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
.#..^.....
........#.
#.........
......#...
",
        );
        let expected = parse_content(
            "\
....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..
        ",
        );
        let updated = walk_the_guard(&content.map, (4, 6));
        display_map(&updated);
        assert_eq!(updated, expected.map);
    }
}

/// How many X are there in the given Map ?
fn fold(map: &Map) -> usize {
    map.iter()
        .flat_map(|line| line.iter().map(|&c| if c == 'X' { 1 } else { 0 }))
        .sum()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_01() {
        let content = parse_content(
            "\
....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..
",
        );
        assert_eq!(fold(&content.map), 41);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = &mut parse_content(&get_file_content("assets/input"));

    println!("Result: {}", fold(&walk_the_guard(&content.map, (48, 71))));
}
