use std::fs;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Clone)]
struct Content {
    left: Vec<usize>,
    right: Vec<usize>,
}

fn parse_content(lines: &str) -> Content {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    lines.split('\n').for_each(|line| {
        if !line.is_empty() {
            let mut left_right = line.split_whitespace();
            let left_value = left_right.next().unwrap().parse::<usize>().unwrap();
            let right_value = left_right.next().unwrap().parse::<usize>().unwrap();
            let left_pos = left.binary_search(&left_value).unwrap_or_else(|e| e);
            left.insert(left_pos, left_value);
            let right_pos = right.binary_search(&right_value).unwrap_or_else(|e| e);
            right.insert(right_pos, right_value);
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
                left: Vec::from_iter(vec![1, 2, 3, 3, 3, 4]),
                right: Vec::from_iter(vec![3, 3, 3, 4, 5, 9]),
            }
        );
    }
}

fn fold_delta(content: &Content) -> usize {
    content
        .right
        .iter()
        .enumerate()
        .map(|(index, right_value)| {
            let left_value = content.left[index];
            let delta: isize = (*right_value as isize) - (left_value as isize);
            delta.unsigned_abs()
        })
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
        assert_eq!(fold_delta(&content), 11);
    }
}

#[allow(clippy::items_after_test_module)]
fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Result: {}", fold_delta(content));
}
