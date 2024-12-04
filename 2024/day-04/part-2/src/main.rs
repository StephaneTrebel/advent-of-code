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

/// X-MAS orientation.
///
/// 4 possible values (A,B,C,D):
///      M.S      S.S      S.M      M.M
///   A: .A.   B: .A.   C: .A.   D: .A.
///      M.S      M.M      S.M      S.S
enum Orientation {
    A,
    B,
    C,
    D,
}

fn find_xmas(input: &Input, position: (&usize, &usize), orientation: &Orientation) -> usize {
    let x: isize = *position.0 as isize;
    let y: isize = *position.1 as isize;
    let n_w = ((x - 1) as usize, (y - 1) as usize);
    let n_e = ((x + 1) as usize, (y - 1) as usize);
    let s_w = ((x - 1) as usize, (y + 1) as usize);
    let s_e = ((x + 1) as usize, (y + 1) as usize);
    let found = {
        match orientation {
            Orientation::A => {
                input
                    .get(n_w.0)
                    .unwrap_or(&vec![])
                    .get(n_w.1)
                    .unwrap_or(&' ')
                    == &'M'
                    && input
                        .get(s_w.0)
                        .unwrap_or(&vec![])
                        .get(s_w.1)
                        .unwrap_or(&' ')
                        == &'S'
                    && input
                        .get(n_e.0)
                        .unwrap_or(&vec![])
                        .get(n_e.1)
                        .unwrap_or(&' ')
                        == &'M'
                    && input
                        .get(s_e.0)
                        .unwrap_or(&vec![])
                        .get(s_e.1)
                        .unwrap_or(&' ')
                        == &'S'
            }
            Orientation::B => {
                input
                    .get(n_w.0)
                    .unwrap_or(&vec![])
                    .get(n_w.1)
                    .unwrap_or(&' ')
                    == &'S'
                    && input
                        .get(s_w.0)
                        .unwrap_or(&vec![])
                        .get(s_w.1)
                        .unwrap_or(&' ')
                        == &'M'
                    && input
                        .get(n_e.0)
                        .unwrap_or(&vec![])
                        .get(n_e.1)
                        .unwrap_or(&' ')
                        == &'S'
                    && input
                        .get(s_e.0)
                        .unwrap_or(&vec![])
                        .get(s_e.1)
                        .unwrap_or(&' ')
                        == &'M'
            }
            Orientation::C => {
                input
                    .get(n_w.0)
                    .unwrap_or(&vec![])
                    .get(n_w.1)
                    .unwrap_or(&' ')
                    == &'S'
                    && input
                        .get(s_w.0)
                        .unwrap_or(&vec![])
                        .get(s_w.1)
                        .unwrap_or(&' ')
                        == &'S'
                    && input
                        .get(n_e.0)
                        .unwrap_or(&vec![])
                        .get(n_e.1)
                        .unwrap_or(&' ')
                        == &'M'
                    && input
                        .get(s_e.0)
                        .unwrap_or(&vec![])
                        .get(s_e.1)
                        .unwrap_or(&' ')
                        == &'M'
            }
            Orientation::D => {
                input
                    .get(n_w.0)
                    .unwrap_or(&vec![])
                    .get(n_w.1)
                    .unwrap_or(&' ')
                    == &'M'
                    && input
                        .get(s_w.0)
                        .unwrap_or(&vec![])
                        .get(s_w.1)
                        .unwrap_or(&' ')
                        == &'M'
                    && input
                        .get(n_e.0)
                        .unwrap_or(&vec![])
                        .get(n_e.1)
                        .unwrap_or(&' ')
                        == &'S'
                    && input
                        .get(s_e.0)
                        .unwrap_or(&vec![])
                        .get(s_e.1)
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
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
",
        );
        assert_eq!(find_xmas(&content.input, (&1, &2), &Orientation::A), 1);
        // Out of bounds test
        assert_eq!(find_xmas(&content.input, (&0, &0), &Orientation::A), 0);
    }
}

fn fold(content: &Content) -> usize {
    content
        .input
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter().enumerate().map(move |(y, c)| {
                if c == &'A' {
                    find_xmas(&content.input, (&x, &y), &Orientation::A)
                        + find_xmas(&content.input, (&x, &y), &Orientation::B)
                        + find_xmas(&content.input, (&x, &y), &Orientation::C)
                        + find_xmas(&content.input, (&x, &y), &Orientation::D)
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
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
",
        );
        assert_eq!(fold(&content), 9);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Result: {}", fold(content));
}
