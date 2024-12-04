use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

type Input = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Clone)]
struct Content {
    input: Input,
}

fn parse_content(lines: &str) -> Content {
    Content {
        input: lines
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
1234
5678
9ABC
",
        );
        assert_eq!(
            content,
            Content {
                input: vec![
                    vec!['1', '2', '3', '4'],
                    vec!['5', '6', '7', '8'],
                    vec!['9', 'A', 'B', 'C']
                ]
            }
        );
    }
}

enum Direction {
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    SouthWest,
    South,
    SouthEast,
}

fn find_xmas(input: &Input, position: (&usize, &usize), direction: &Direction) -> usize {
    let x: isize = *position.0 as isize;
    let y: isize = *position.1 as isize;
    let found = {
        match direction {
            Direction::NorthWest => {
                input
                    .get((x - 1) as usize)
                    .unwrap_or(&vec![])
                    .get((y - 1) as usize)
                    .unwrap_or(&' ')
                    == &'M'
                    && input
                        .get((x - 2) as usize)
                        .unwrap_or(&vec![])
                        .get((y - 2) as usize)
                        .unwrap_or(&' ')
                        == &'A'
                    && input
                        .get((x - 3) as usize)
                        .unwrap_or(&vec![])
                        .get((y - 3) as usize)
                        .unwrap_or(&' ')
                        == &'S'
            }
            Direction::North => {
                input
                    .get(x as usize)
                    .unwrap_or(&vec![])
                    .get((y - 1) as usize)
                    .unwrap_or(&' ')
                    == &'M'
                    && input
                        .get(x as usize)
                        .unwrap_or(&vec![])
                        .get((y - 2) as usize)
                        .unwrap_or(&' ')
                        == &'A'
                    && input
                        .get(x as usize)
                        .unwrap_or(&vec![])
                        .get((y - 3) as usize)
                        .unwrap_or(&' ')
                        == &'S'
            }
            Direction::NorthEast => {
                input
                    .get((x + 1) as usize)
                    .unwrap_or(&vec![])
                    .get((y - 1) as usize)
                    .unwrap_or(&' ')
                    == &'M'
                    && input
                        .get((x + 2) as usize)
                        .unwrap_or(&vec![])
                        .get((y - 2) as usize)
                        .unwrap_or(&' ')
                        == &'A'
                    && input
                        .get((x + 3) as usize)
                        .unwrap_or(&vec![])
                        .get((y - 3) as usize)
                        .unwrap_or(&' ')
                        == &'S'
            }
            Direction::West => {
                input
                    .get((x - 1) as usize)
                    .unwrap_or(&vec![])
                    .get(y as usize)
                    .unwrap_or(&' ')
                    == &'M'
                    && input
                        .get((x - 2) as usize)
                        .unwrap_or(&vec![])
                        .get(y as usize)
                        .unwrap_or(&' ')
                        == &'A'
                    && input
                        .get((x - 3) as usize)
                        .unwrap_or(&vec![])
                        .get(y as usize)
                        .unwrap_or(&' ')
                        == &'S'
            }
            Direction::East => {
                input
                    .get((x + 1) as usize)
                    .unwrap_or(&vec![])
                    .get(y as usize)
                    .unwrap_or(&' ')
                    == &'M'
                    && input
                        .get((x + 2) as usize)
                        .unwrap_or(&vec![])
                        .get(y as usize)
                        .unwrap_or(&' ')
                        == &'A'
                    && input
                        .get((x + 3) as usize)
                        .unwrap_or(&vec![])
                        .get(y as usize)
                        .unwrap_or(&' ')
                        == &'S'
            }
            Direction::SouthWest => {
                input
                    .get((x - 1) as usize)
                    .unwrap_or(&vec![])
                    .get((y + 1) as usize)
                    .unwrap_or(&' ')
                    == &'M'
                    && input
                        .get((x - 2) as usize)
                        .unwrap_or(&vec![])
                        .get((y + 2) as usize)
                        .unwrap_or(&' ')
                        == &'A'
                    && input
                        .get((x - 3) as usize)
                        .unwrap_or(&vec![])
                        .get((y + 3) as usize)
                        .unwrap_or(&' ')
                        == &'S'
            }
            Direction::South => {
                input
                    .get(x as usize)
                    .unwrap_or(&vec![])
                    .get((y + 1) as usize)
                    .unwrap_or(&' ')
                    == &'M'
                    && input
                        .get(x as usize)
                        .unwrap_or(&vec![])
                        .get((y + 2) as usize)
                        .unwrap_or(&' ')
                        == &'A'
                    && input
                        .get(x as usize)
                        .unwrap_or(&vec![])
                        .get((y + 3) as usize)
                        .unwrap_or(&' ')
                        == &'S'
            }
            Direction::SouthEast => {
                input
                    .get((x + 1) as usize)
                    .unwrap_or(&vec![])
                    .get((y + 1) as usize)
                    .unwrap_or(&' ')
                    == &'M'
                    && input
                        .get((x + 2) as usize)
                        .unwrap_or(&vec![])
                        .get((y + 2) as usize)
                        .unwrap_or(&' ')
                        == &'A'
                    && input
                        .get((x + 3) as usize)
                        .unwrap_or(&vec![])
                        .get((y + 3) as usize)
                        .unwrap_or(&' ')
                        == &'S'
            }
        }
    };
    if found {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests_find_xmas {
    use super::*;

    #[test]
    fn find_xmas_01() {
        let content = parse_content(
            "\
S00S00S
0A0A0A0
00MMM00
SAMXMAS
00MMM00
0A0A0A0
S00S00S
",
        );
        assert_eq!(
            find_xmas(&content.input, (&3, &3), &Direction::NorthWest),
            1
        );
        assert_eq!(find_xmas(&content.input, (&3, &3), &Direction::North), 1);
        assert_eq!(
            find_xmas(&content.input, (&3, &3), &Direction::NorthEast),
            1
        );
        assert_eq!(find_xmas(&content.input, (&3, &3), &Direction::West), 1);
        assert_eq!(find_xmas(&content.input, (&3, &3), &Direction::East), 1);
        assert_eq!(
            find_xmas(&content.input, (&3, &3), &Direction::SouthWest),
            1
        );
        assert_eq!(find_xmas(&content.input, (&3, &3), &Direction::South), 1);
        assert_eq!(
            find_xmas(&content.input, (&3, &3), &Direction::SouthEast),
            1
        );
        // Out of bounds test
        assert_eq!(
            find_xmas(&content.input, (&0, &0), &Direction::NorthWest),
            0
        );
    }
}

fn fold(content: &Content) -> usize {
    content
        .input
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter().enumerate().map(move |(y, c)| {
                if c == &'X' {
                    find_xmas(&content.input, (&x, &y), &Direction::NorthWest)
                        + find_xmas(&content.input, (&x, &y), &Direction::North)
                        + find_xmas(&content.input, (&x, &y), &Direction::NorthEast)
                        + find_xmas(&content.input, (&x, &y), &Direction::West)
                        + find_xmas(&content.input, (&x, &y), &Direction::East)
                        + find_xmas(&content.input, (&x, &y), &Direction::SouthWest)
                        + find_xmas(&content.input, (&x, &y), &Direction::South)
                        + find_xmas(&content.input, (&x, &y), &Direction::SouthEast)
                } else {
                    0
                }
            })
        })
        .sum()
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_01() {
        let content = parse_content(
            "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
        );
        assert_eq!(fold(&content), 18);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Result: {}", fold(content));
}
