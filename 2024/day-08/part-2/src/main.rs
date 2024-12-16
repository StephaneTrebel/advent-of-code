use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, fs};

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

/// There are now multiple antinodes per antenna pair
fn compute_antinodes((xa, ya): &Coords, (xb, yb): &Coords, (max_x, max_y): Coords) -> Vec<Coords> {
    let delta_x: isize = *xb as isize - *xa as isize;
    let delta_y: isize = *yb as isize - *ya as isize;

    let mut antinodes: Vec<Coords> = vec![];

    // First side (arbitrarily chosen)
    let mut x: isize = *xb as isize - delta_x;
    let mut y: isize = *yb as isize - delta_y;
    while x >= 0 && x < max_x as isize && y >= 0 && y < max_y as isize {
        antinodes.push((x as usize, y as usize));
        x -= delta_x;
        y -= delta_y;
    }

    // Other side (since we're going on both sides of the axis defined by both antennas)
    let mut x: isize = *xa as isize + delta_x;
    let mut y: isize = *ya as isize + delta_y;
    while x >= 0 && x < max_x as isize && y >= 0 && y < max_y as isize {
        antinodes.push((x as usize, y as usize));
        x += delta_x;
        y += delta_y;
    }

    antinodes
}

#[cfg(test)]
mod tests_compute_antinodes {
    use super::*;

    #[test]
    fn compute_antinodes_01() {
        assert_eq!(
            compute_antinodes(&(0, 0), &(1, 2), (9, 9)),
            vec![(0, 0), (1, 2), (2, 4), (3, 6), (4, 8)]
        );
    }
}

fn fold(content: &Content) -> usize {
    let mut antennas: HashMap<char, Vec<Coords>> = HashMap::new();
    let bounding_rect = content.get_bounding_rect();

    // Parse map to retrieve all antennas coordinates
    content.map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, &freq)| {
            if freq != '.' && freq != '#' {
                match antennas.get(&freq) {
                    Some(antennas_position_list) => {
                        let mut updated_list = antennas_position_list.clone();
                        updated_list.push((x, y));
                        antennas.insert(freq, updated_list);
                    }
                    None => {
                        antennas.insert(freq, vec![(x, y)]);
                    }
                }
            }
        })
    });

    // Count all antinodes
    let mut antinodes: Vec<Coords> = antennas
        .iter()
        .flat_map(|(_, coords)| {
            coords
                .iter()
                .combinations(2)
                .unique()
                .flat_map(|c| compute_antinodes(c[0], c[1], bounding_rect))
        })
        .unique()
        .collect();

    antinodes.sort_by(|(xa, ya), (xb, yb)| match ya.cmp(yb) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => xa.cmp(xb),
        Ordering::Greater => Ordering::Greater,
    });
    antinodes.len()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_a() {
        let content = Content::from(
            "\
    T....#....
    ...T......
    .T....#...
    .........#
    ..#.......
    ..........
    ...#......
    ..........
    ....#.....
    ..........
    ",
        );
        assert_eq!(fold(&content), 9);
    }

    #[test]
    fn fold_b() {
        let content = Content::from(
            "\
    #.........
    ..........
    .#........
    ..........
    ..a.......
    ..........
    ...a......
    ..........
    ....#.....
    ..........
    ",
        );
        assert_eq!(fold(&content), 5);
    }

    #[test]
    fn fold_c() {
        let content = Content::from(
            "\
    T....#....
    ...T......
    .T....#...
    .........#
    ..a.......
    ..........
    ...a......
    ..........
    ....#.....
    ..........
    ",
        );
        assert_eq!(fold(&content), 9);
    }

    #[test]
    fn fold_d() {
        let content = Content::from(
            "\
    .#....#....#
    ...#....0...
    .....0....#.
    ..#....0....
    ....0....#..
    .#....#....#
    ...#........
    #....#......
    ..#.........
    ....#.......
    .#..........
    ...#........
    ",
        );
        assert_eq!(fold(&content), 21);
    }

    #[test]
    fn fold_e() {
        let content = Content::from(
            "\
    #...........
    .#.#........
    ..#.#.......
    ...#........
    ....#.......
    .....#A.....
    ......#.....
    .......#....
    ........A...
    .........A..
    ..........#.
    ..........##
    ",
        );
        assert_eq!(fold(&content), 16);
    }

    #[test]
    fn fold_final() {
        let content = Content::from(
            "\
    ##....#....#
    .#.#....0...
    ..#.#0....#.
    ..##...0....
    ....0....#..
    .#...#A....#
    ...#..#.....
    #....#.#....
    ..#.....A...
    ....#....A..
    .#........#.
    ...#......##
    ",
        );
        assert_eq!(fold(&content), 34);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = Content::from(get_file_content("assets/input"));

    println!("Result: {}", fold(&content));
}
