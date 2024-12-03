use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Clone)]
struct Content {
    list: Vec<Vec<usize>>,
}

fn parse_content(lines: &str) -> Content {
    Content {
        list: lines
            .split('\n')
            .map(|line| {
                line.split_whitespace()
                    .map(|e| e.parse().unwrap())
                    .collect::<Vec<usize>>()
            })
            .filter(|l| !l.is_empty())
            .collect::<Vec<Vec<usize>>>(),
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
        );
        assert_eq!(
            content,
            Content {
                list: vec![
                    vec![7, 6, 4, 2, 1],
                    vec![1, 2, 7, 8, 9],
                    vec![9, 7, 6, 2, 1],
                    vec![1, 3, 2, 4, 5],
                    vec![8, 6, 4, 4, 1],
                    vec![1, 3, 6, 7, 9],
                ]
            }
        );
    }
}

fn is_monotonic_and_bounded(list: &[usize]) -> bool {
    if list.len() < 2 {
        return true;
    }
    let direction: isize = list[0] as isize - list[1] as isize;
    if direction.unsigned_abs() > 3 {
        return false;
    }
    for index in 1..list.len() - 1 {
        let temp: isize = list[index] as isize - list[index + 1] as isize;
        if temp.unsigned_abs() > 3 || temp.signum() != direction.signum() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests_is_monotonic_and_bounded {
    use super::*;

    #[test]
    fn is_monotonic_and_bounded_01() {
        assert!(is_monotonic_and_bounded(&[1, 2, 3]));
        assert!(!is_monotonic_and_bounded(&[1, 2, 7]));
        assert!(is_monotonic_and_bounded(&[3, 2, 1]));
        assert!(!is_monotonic_and_bounded(&[7, 2, 1]));
    }
}

fn fold_delta(content: &Content) -> usize {
    content
        .list
        .iter()
        .map(|levels| is_monotonic_and_bounded(levels))
        .filter(|b| *b)
        .count()
}

#[cfg(test)]
mod tests_fold_delta {
    use super::*;

    #[test]
    fn fold_delta_01() {
        let content = parse_content(
            "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
        );
        assert_eq!(fold_delta(&content), 2);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Result: {}", fold_delta(content));
}
