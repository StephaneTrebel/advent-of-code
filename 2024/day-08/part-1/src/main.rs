use itertools::Itertools;
use std::{collections::HashMap, fs};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

type Coords = (usize, usize);
type Map = Vec<Vec<char>>;
#[derive(Debug, PartialEq, Clone)]
struct Content {
    map: Map,
}

impl Content {
    #[allow(dead_code)]
    fn display_map(&self) {
        self.map.iter().for_each(|line| {
            line.iter().for_each(|c| print!("{}", c));
            println!()
        });
    }

    fn get_bounding_rect(&self) -> (usize, usize) {
        (self.map[0].len(), self.map.len())
    }
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Self {
            map: value
                .split_whitespace()
                .map(|line| line.chars().collect())
                .collect(),
        }
    }
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self {
            map: value
                .split_whitespace()
                .map(|line| line.chars().collect())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests_content {
    use super::*;

    #[test]
    fn content_from() {
        let content = Content::from(
            "\
..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........
",
        );
        assert_eq!(
            content,
            Content {
                map: vec![
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', 'a', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', 'a', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                    vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.']
                ]
            }
        );
    }
}

#[derive(Debug)]
enum Antinodes {
    None,
    One(Coords),
    Two(Coords, Coords),
}

impl PartialEq for Antinodes {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Antinodes::None, Antinodes::None) => true,
            (Antinodes::One(s), Antinodes::One(o)) => s == o,
            (Antinodes::Two(sa, sb), Antinodes::Two(oa, ob)) => {
                (sa == oa && sb == ob) || (sa == ob && sb == oa)
            }
            (_, _) => true,
        }
    }
}

/// There are only two antinodes per antenna pair.
///
/// ("a" are antennas, "#" are their antinodes):
/// # . . .
/// . . . .
/// . a . .
/// . . . .
/// . . a .
/// . . . .
/// . . . #
///
/// bounding_rect is assumed to start at (0,0) and to end at (max_x, max_y)
fn compute_antinode((xa, ya): &Coords, (xb, yb): &Coords, (max_x, max_y): Coords) -> Antinodes {
    let delta_x: isize = *xb as isize - *xa as isize;
    let delta_y: isize = *yb as isize - *ya as isize;

    let first_x: isize = *xa as isize - delta_x;
    let first_y: isize = *ya as isize - delta_y;

    let second_x: isize = *xb as isize + delta_x;
    let second_y: isize = *yb as isize + delta_y;

    let first = {
        if first_x >= 0 && first_x < max_x as isize && first_y >= 0 && first_y < max_y as isize {
            Some((first_x as usize, first_y as usize))
        } else {
            None
        }
    };

    let second = {
        if second_x >= 0 && second_x < max_x as isize && second_y >= 0 && second_y < max_y as isize
        {
            Some((second_x as usize, second_y as usize))
        } else {
            None
        }
    };

    match (first, second) {
        (None, None) => Antinodes::None,
        (None, Some(s)) => Antinodes::One(s),
        (Some(f), None) => Antinodes::One(f),
        (Some(f), Some(s)) => Antinodes::Two(f, s),
    }
}

#[cfg(test)]
mod tests_compute_antinode {
    use super::*;

    #[test]
    fn compute_antinode_01() {
        assert_eq!(
            compute_antinode(&(10, 10), &(11, 11), (100, 100)),
            Antinodes::Two((9, 9), (12, 12))
        );
    }
}

fn fold(content: &Content) -> usize {
    let mut antennas: HashMap<char, Vec<Coords>> = HashMap::new();

    // Parse map to retrieve all antennas coordinates
    content.map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, &c)| {
            if c != '.' {
                match antennas.get(&c) {
                    Some(v) => {
                        let mut updated_v = v.clone();
                        updated_v.push((x, y));
                        antennas.insert(c, updated_v);
                    }
                    None => {
                        antennas.insert(c, vec![(x, y)]);
                    }
                }
            }
        })
    });

    // Count all antinodes
    antennas
        .iter()
        .flat_map(|(_, coords)| {
            coords
                .iter()
                .combinations(2)
                .unique()
                .map(|c| compute_antinode(c[0], c[1], content.get_bounding_rect()))
        })
        .flat_map(|antinodes| match antinodes {
            Antinodes::None => vec![],
            Antinodes::One(a) => vec![a],
            Antinodes::Two(a, b) => vec![a, b],
        })
        .unique()
        .count()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_01() {
        let content = Content::from(
            "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
        );
        assert_eq!(fold(&content), 14);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = Content::from(get_file_content("assets/input"));

    println!("Result: {}", fold(&content));
}
