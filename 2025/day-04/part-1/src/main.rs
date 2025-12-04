use std::{collections::HashSet, fs};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {file_path}");
    fs::read_to_string(file_path).expect("Cannot load file")
}

fn parse_content(lines: &str) -> Vec<Vec<char>> {
    lines
        .split_whitespace()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests_parse_content {
    use std::vec;

    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            "\
...
...
...
",
        );
        assert_eq!(
            content,
            vec![['.', '.', '.'], ['.', '.', '.'], ['.', '.', '.']]
        );
    }

    #[test]
    fn parse_content_02() {
        let content = parse_content(
            "\
.@
@.
",
        );
        assert_eq!(content, vec![vec!['.', '@'], vec!['@', '.']]);
    }

    #[test]
    fn parse_content_final() {
        let content = parse_content(
            "\
............
...@@.@@@@..
.@@@.@.@.@@.
.@@@@@.@.@@.
.@.@@@@..@..
.@@.@@@@.@@.
..@@@@@@@.@.
.@.@.@.@@@..
.@.@@@.@@@@.
..@@@@@@@@..
.@.@.@@@.@..
............
    ",
        );
        assert_eq!(
            content,
            [
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '@', '@', '.', '@', '@', '@', '@', '.', '.'],
                ['.', '@', '@', '@', '.', '@', '.', '@', '.', '@', '@', '.'],
                ['.', '@', '@', '@', '@', '@', '.', '@', '.', '@', '@', '.'],
                ['.', '@', '.', '@', '@', '@', '@', '.', '.', '@', '.', '.'],
                ['.', '@', '@', '.', '@', '@', '@', '@', '.', '@', '@', '.'],
                ['.', '.', '@', '@', '@', '@', '@', '@', '@', '.', '@', '.'],
                ['.', '@', '.', '@', '.', '@', '.', '@', '@', '@', '.', '.'],
                ['.', '@', '.', '@', '@', '@', '.', '@', '@', '@', '@', '.'],
                ['.', '.', '@', '@', '@', '@', '@', '@', '@', '@', '.', '.'],
                ['.', '@', '.', '@', '.', '@', '@', '@', '.', '@', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.']
            ]
        );
    }
}

fn is_reachable(map: &[Vec<char>], x: usize, y: usize) -> bool {
    let positions = [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
        (x - 1, y),
        (x + 1, y),
    ];
    let count = positions.iter().filter(|(x, y)| map[*x][*y] == '@').count();

    count < 4
}

#[cfg(test)]
mod tests_is_reachable {
    use super::*;

    #[test]
    fn is_reachable_01() {
        let content = parse_content(
            "\
...
.@.
...
",
        );
        assert!(is_reachable(&content, 1, 1));
    }
}

fn display_map(map: &[Vec<char>], coords: &HashSet<(usize, usize)>) {
    for x in 1..(map.len() - 1) {
        let line = &map[x];
        for y in 1..(line.len() - 1) {
            let position = &line[y];
            if coords.contains(&(x, y)) {
                print!("X");
            } else {
                print!("{position}");
            }
        }
        println!();
    }
}

fn fold(map: &[Vec<char>]) -> usize {
    let mut sum: usize = 0;
    let mut coords: HashSet<(usize, usize)> = HashSet::new();
    for x in 1..(map.len() - 1) {
        let line = &map[x];
        for y in 1..(line.len() - 1) {
            let position = &line[y];
            if *position == '@' && is_reachable(map, x, y) {
                sum += 1;
                coords.insert((x, y));
            }
        }
    }

    display_map(map, &coords);

    sum
}

#[cfg(test)]
mod tests_fold {
    use super::*;

    #[test]
    fn fold_final() {
        let content = parse_content(
            "\
............
...@@.@@@@..
.@@@.@.@.@@.
.@@@@@.@.@@.
.@.@@@@..@..
.@@.@@@@.@@.
..@@@@@@@.@.
..@.@.@.@@@.
.@.@@@.@@@@.
..@@@@@@@@..
.@.@.@@@.@..
............
",
        );
        assert_eq!(fold(&content), 13);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let binding = get_file_content("assets/input");
    let content = &parse_content(&binding);
    println!("Result: {}", fold(content));
}
