use std::{collections::HashMap, fs};

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Clone)]
struct Content {
    left: Vec<usize>,
    right: HashMap<usize, usize>,
}

fn parse_content(lines: &str) -> Content {
    let mut left: Vec<usize> = Vec::new();
    let mut right: HashMap<usize, usize> = HashMap::new();
    lines.split('\n').for_each(|line| {
        if !line.is_empty() {
            let mut left_right = line.split_whitespace();
            let left_value = left_right.next().unwrap().parse::<usize>().unwrap();
            left.push(left_value);

            let right_value = left_right.next().unwrap().parse::<usize>().unwrap();
            let right_count = right.get(&right_value).unwrap_or(&0);
            right.insert(right_value, right_count + 1);
        }
    });
    Content { left, right }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            "\
3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(
            content,
            Content {
                left: Vec::from_iter(vec![3, 4, 2, 1, 3, 3]),
                right: HashMap::from_iter(vec![(3, 3), (4, 1), (5, 1), (9, 1)]),
            }
        );
    }
}

fn fold(content: &Content) -> usize {
    content
        .left
        .iter()
        .map(|left_value| left_value * content.right.get(left_value).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests_fold_delta {
    use super::*;

    #[test]
    fn fold_delta_01() {
        let content = parse_content(
            "\
3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(fold(&content), 31);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Result: {}", fold(content));
}
