use std::{
    char,
    collections::HashMap,
    fmt::{self, Display},
    fs,
};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    max_x: usize,
    max_y: usize,
    starting_point: (usize, usize),
    layout: HashMap<(usize, usize), char>,
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "({}, {})", self.longitude, self.latitude)
        for x in 0..self.max_x {
            for y in 0..=self.max_y {
                write!(
                    f,
                    "{}",
                    self.layout
                        .get(&(x, y))
                        .unwrap_or_else(|| panic!("Unknown location: {x},{y}"))
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_content(content: &str) -> Map {
    let mut layout: HashMap<(usize, usize), char> = HashMap::new();
    let max_x = content.split_whitespace().count();
    let mut max_y = 0;
    let mut starting_point = (0, 0);
    content.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            if max_y == 0 {
                max_y = line.len() - 1;
            }
            if c == 'S' {
                starting_point = (x, y);
            }
            layout.insert((x, y), c);
        });
    });
    // dbg!(max_x);
    // dbg!(max_y);

    // println!("{map}");
    Map {
        max_x,
        max_y,
        starting_point,
        layout,
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_final() {
        let map = parse_content(
            "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );

        assert_eq!(
            map.to_string(),
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
",
        );
    }
}

fn launch_beam(map: &mut Map) -> u64 {
    println!("Entering launch_beam");
    let mut split = 0;
    let mut queue: Vec<(usize, usize)> = vec![map.starting_point];
    // println!("Starting Point: {:?}", map.starting_point);

    while let Some((x, y)) = queue.pop() {
        let tmp = map.layout.clone();
        let current_point = map.layout.get(&(x, y));
        let below = (x + 1, y);
        let char_below = tmp.get(&below);
        // println!(
            // "Current point({},{}): {:?}, Char below({},{}): {:?}",
            // x,
            // y,
            // current_point,
            // x + 1,
            // y,
            // char_below
        // );
        match (current_point, char_below) {
            (Some(c), _) if *c == 'S' => {
                // println!("Coming from Starting point, continuing the beam");
                map.layout.insert(below, '|');
                queue.push(below);
            }
            (Some(c), Some('.')) if *c == '|' => {
                // println!("Continuing the beam");
                map.layout.insert(below, '|');
                queue.push(below);
            }
            (Some(c), Some('^')) if *c == '|' => {
                let mut new_split = false;
                // println!("Splitting the beam");
                let left = (x + 1, y - 1);
                if let Some('.') = map.layout.get(&left) {
                    map.layout.insert(left, '|');
                    new_split = true;
                    // println!("New split on left. Split count: {split}");
                    queue.push(left);
                }
                let right = (x + 1, y + 1);
                if let Some('.') = map.layout.get(&right) {
                    map.layout.insert(right, '|');
                    new_split = true;
                    // println!("New split on right. Split count: {split}");
                    queue.push(right);
                }
                if new_split {
                    split += 1;
                }
                // println!("{map}");
            }
            (None, _) | (_, None) => {
                // println!("Reaching the bottom of the map");
            }
            _ => {
                // println!("Any other case");
            }
        }
    }

    println!("{map}");

    split
}

#[cfg(test)]
mod tests_launch_beam {
    use super::*;

    #[test]
    fn launch_beam_final() {
        let mut map = parse_content(
            "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );
        assert_eq!(launch_beam(&mut map), 21);
    }
}

// fn fold(map: &Map) -> u64 {
// let mut split_count = 0;
// for x in 0..map.max_x {
// for y in 0..=map.max_y {
// match map.layout.get(&(x, y)) {
// Some('^') if x > 0 => {
// if let Some('|') = map.layout.get(&(x - 1, y)) {
// split_count += 1;
// }
// }
// _ => {}
// }
// }
// }
// split_count
// }

// #[cfg(test)]
// mod tests_fold {
// use super::*;

// #[test]
// fn fold_final() {
// let mut map = parse_content(
// "\
// .......S.......
// ...............
// .......^.......
// ...............
// ......^.^......
// ...............
// .....^.^.^.....
// ...............
// ....^.^...^....
// ...............
// ...^.^...^.^...
// ...............
// ..^...^.....^..
// ...............
// .^.^.^.^.^...^.
// ...............",
// );
// launch_beam(&mut map);
// assert_eq!(fold(&map), 21);
// }
// }

#[allow(clippy::items_after_test_module)]
fn main() {
    let binding = get_file_content("assets/input");
    let mut map = parse_content(&binding);
    println!("Result: {}", launch_beam(&mut map));
}
